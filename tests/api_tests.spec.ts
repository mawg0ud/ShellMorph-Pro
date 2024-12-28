import supertest from "supertest";
import fs from "fs";
import path from "path";
import app from "../src/api/server"; // Assuming the server exports an `app` instance

const request = supertest(app);

describe("API Integration Tests", () => {
    const inputFilePath = path.join(__dirname, "test_input.txt");
    const encryptedFilePath = path.join(__dirname, "test_output.enc");
    const decryptedFilePath = path.join(__dirname, "test_decrypted.txt");
    const key = "supersecretkey";

    beforeAll(() => {
        fs.writeFileSync(inputFilePath, "This is test data for encryption.");
    });

    afterAll(() => {
        [inputFilePath, encryptedFilePath, decryptedFilePath].forEach((file) => {
            if (fs.existsSync(file)) {
                fs.unlinkSync(file);
            }
        });
    });

    test("POST /encrypt - Encrypt a file", async () => {
        const res = await request.post("/encrypt").send({
            inputPath: inputFilePath,
            outputPath: encryptedFilePath,
            key,
        });

        expect(res.status).toBe(200);
        expect(res.body.message).toBe("File encrypted successfully");
        expect(fs.existsSync(encryptedFilePath)).toBe(true);
    });

    test("POST /encrypt - Decrypt a file (re-encrypt with same key)", async () => {
        const res = await request.post("/encrypt").send({
            inputPath: encryptedFilePath,
            outputPath: decryptedFilePath,
            key,
        });

        expect(res.status).toBe(200);
        expect(res.body.message).toBe("File encrypted successfully");
        expect(fs.existsSync(decryptedFilePath)).toBe(true);

        const decryptedContent = fs.readFileSync(decryptedFilePath, "utf8");
        expect(decryptedContent).toBe("This is test data for encryption.");
    });

    test("POST /execute - Execute shellcode", async () => {
        const shellcodeFilePath = path.join(__dirname, "test_shellcode.bin");
        fs.writeFileSync(shellcodeFilePath, Buffer.from([0xB8, 0x2A, 0x00, 0x00, 0x00, 0xCD, 0x80]));

        const res = await request.post("/execute").send({
            shellcodePath: shellcodeFilePath,
        });

        expect(res.status).toBe(200);
        expect(res.body.message).toBe("Shellcode executed successfully");

        fs.unlinkSync(shellcodeFilePath);
    });
});
