import os
import time
import random
import string
from concurrent.futures import ThreadPoolExecutor
from subprocess import run

def generate_random_file(file_path, size_mb):
    """Generate a random binary file of the specified size."""
    with open(file_path, "wb") as f:
        f.write(os.urandom(size_mb * 1024 * 1024))

def stress_test(file_path, output_path, key, iterations):
    """Perform encryption stress test."""
    for i in range(iterations):
        command = f"./shellmorph-cli encrypt {file_path} {output_path} {key}"
        result = run(command, shell=True)
        if result.returncode != 0:
            print(f"Iteration {i + 1}: Encryption failed!")
        else:
            print(f"Iteration {i + 1}: Encryption succeeded!")

def parallel_stress_test(num_files, file_size_mb, key, iterations):
    """Perform stress tests in parallel."""
    os.makedirs("stress_test", exist_ok=True)
    files = [
        (f"stress_test/input_{i}.bin", f"stress_test/output_{i}.enc")
        for i in range(num_files)
    ]

    # Generate random files
    for input_file, _ in files:
        generate_random_file(input_file, file_size_mb)

    # Run stress tests in parallel
    with ThreadPoolExecutor() as executor:
        for input_file, output_file in files:
            executor.submit(stress_test, input_file, output_file, key, iterations)

    # Cleanup
    for input_file, output_file in files:
        os.remove(input_file)
        if os.path.exists(output_file):
            os.remove(output_file)

if __name__ == "__main__":
    NUM_FILES = 5
    FILE_SIZE_MB = 50
    KEY = "".join(random.choices(string.ascii_letters + string.digits, k=16))
    ITERATIONS = 10

    print("Starting stress test...")
    start_time = time.time()
    parallel_stress_test(NUM_FILES, FILE_SIZE_MB, KEY, ITERATIONS)
    print(f"Stress test completed in {time.time() - start_time:.2f} seconds.")
