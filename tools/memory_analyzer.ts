import os from "os";
import { exec } from "child_process";

/**
 * Logs the current memory usage.
 */
function logMemoryUsage() {
    const memoryUsage = process.memoryUsage();
    console.log("Memory Usage (MB):");
    console.log(`  RSS: ${(memoryUsage.rss / 1024 / 1024).toFixed(2)}`);
    console.log(`  Heap Total: ${(memoryUsage.heapTotal / 1024 / 1024).toFixed(2)}`);
    console.log(`  Heap Used: ${(memoryUsage.heapUsed / 1024 / 1024).toFixed(2)}`);
}

/**
 * Runs a shell command and monitors its memory usage.
 * @param command - The shell command to execute.
 */
function monitorCommand(command: string) {
    const interval = setInterval(logMemoryUsage, 1000);

    const child = exec(command, (error, stdout, stderr) => {
        if (error) {
            console.error(`Command failed: ${stderr}`);
        } else {
            console.log(`Command output: ${stdout}`);
        }
        clearInterval(interval);
    });

    child.on("close", () => {
        clearInterval(interval);
    });
}

// Example usage
if (require.main === module) {
    const command = "./shellmorph-cli exec test_shellcode.bin";
    console.log("Starting memory analyzer...");
    monitorCommand(command);
}
