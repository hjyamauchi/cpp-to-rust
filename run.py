#!/usr/bin/python
# Run the C++ and the Rust versions of the benchmarks

import os
import shlex
import shutil
import subprocess
import time

num_iter = 1

benchmarks = ["ackermann",
              "ary",
              "ary2",
              "ary3",
              "fibo",
              "hash",
              "hash2",
              "heapsort",
              "lists",
              "lists1",
              "matrix",
              "methcall",
              "moments",
              "nestedloop",
              "objinst",
              "random",
              "sieve",
              "strcat"]

cwd = os.getcwd()

output = {}

time_cpp = {}
time_rust = {}

def run_command(command):
    proc = subprocess.run(command, capture_output=True, text=True, shell=True)
    return proc.returncode, proc.stdout, proc.stderr
    
def run_bench_with_time(command, num_iter):
    sum_time = 0
    for i in range(num_iter):
        start_time = time.time()
        process = subprocess.Popen(command, shell=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
        stdout, stderr = process.communicate()
        end_time = time.time()
        execution_time = end_time - start_time
        sum_time += execution_time
        returncode = process.returncode
        if returncode != 0:
            print(f"Command failed: {command}")
            break
    avg_time = sum_time / num_iter
    return returncode, stdout.decode(), stderr.decode(), avg_time

# The C++ benchmarks
print("Fetching the C++ benchmarks...")
os.makedirs(f"{cwd}/benchmarks/cpp", exist_ok=True)
os.chdir(f"{cwd}/benchmarks/cpp")

# Clone
if os.path.exists(f"{cwd}/benchmarks/cpp/llvm-test-suite"):
    returncode, stdout, stderr = run_command("cd llvm-test-suite; git checkout a45d1629b312c9319e16386132a58943572df43f; cd ..")
else:
    returncode, stdout, stderr = run_command("git clone https://github.com/llvm/llvm-test-suite; cd llvm-test-suite; git checkout a45d1629b312c9319e16386132a58943572df43f; cd ..")
#print(stdout)
#print(stderr)
if returncode != 0:
    print("Failed to clone llvm-test-suite")
    exit(1)

# Build
print("Building the C++ benchmarks...")
for bench in benchmarks:
    returncode, stdout, stderr = run_command(f"clang++ -O2 llvm-test-suite/SingleSource/Benchmarks/Shootout-C++/{bench}.cpp -o {bench}")
    #print(stdout)
    #print(stderr)
    if returncode != 0:
        print(f"Failed to build {bench}.cpp")
        exit(1)

# Run
print("Running the C++ benchmarks...")
for bench in benchmarks:
    returncode, stdout, stderr, execution_time = run_bench_with_time(f"./{bench}", num_iter)
    if returncode != 0:
        print(f"Failed to run {bench}")
        exit(1)
    output[bench] = stdout
    time_cpp[bench] = execution_time

# The rust benchmarks
os.makedirs(f"{cwd}/benchmarks/rust", exist_ok=True)
os.chdir(f"{cwd}/benchmarks/rust")

# Build
print("Building the Rust benchmarks...")
for bench in benchmarks:
    returncode, stdout, stderr = run_command(f"rustc -O {cwd}/src/{bench}.rs -o {bench}")
    #print(stdout)
    if stderr:
        print(stderr)
    if returncode != 0:
        print(f"Failed to build {bench}.rs")
        exit(1)

# Run
print("Running the Rust benchmarks...")
for bench in benchmarks:
    returncode, stdout, stderr, execution_time = run_bench_with_time(f"./{bench}", num_iter)
    if returncode != 0:
        print(f"Failed to run {bench}")
        exit(1)
    if output[bench] != stdout:
        print(f"The output mismatch for {bench}")
        print(f"cpp {output[bench]}")
        print(f"rust {proc.stdout}")
        exit(1)
    time_rust[bench] = execution_time

# Show the results
width = 16
print(f"|Program         |C++  (s)|Rust (s)|Ratio (x)|")
print(f"|----------------|-------:|-------:|--------:|")
for bench in benchmarks:
    print(f"|{bench:<{width}}|{time_cpp[bench]:.4f}|{time_rust[bench]:.4f}|{time_rust[bench]/time_cpp[bench]:.4f}|")
