import express from "express";
import { exec } from "child_process";
import * as fs from "fs";
import * as util from "util";

const app = express();
const port = 8080;
const execAsync = util.promisify(exec);

// Middleware
app.use(express.json());
app.use(express.urlencoded({ extended: true }));

// Encrypt Endpoint
app.post("/encrypt", async (req, res) => {
    try {
        const { inputPath, outputPath, key } = req.body;
        if (!inputPath || !outputPath || !key) {
            return res.status(400).json({ error: "Missing parameters" });
        }

        const command = `shellmorph-cli encrypt "${inputPath}" "${outputPath}" "${key}"`;
        await execAsync(command);
        res.status(200).json({ message: "File encrypted successfully" });
    } catch (error) {
        res.status(500).json({ error: error.message });
    }
});

// Execute Shellcode Endpoint
app.post("/execute", async (req, res) => {
    try {
        const { shellcodePath } = req.body;
        if (!shellcodePath) {
            return res.status(400).json({ error: "Missing parameters" });
        }

        const command = `shellmorph-cli exec "${shellcodePath}"`;
        await execAsync(command);
        res.status(200).json({ message: "Shellcode executed successfully" });
    } catch (error) {
        res.status(500).json({ error: error.message });
    }
});

// Start Server
app.listen(port, () => {
    console.log(`ShellMorph Pro API server running on port ${port}`);
});
