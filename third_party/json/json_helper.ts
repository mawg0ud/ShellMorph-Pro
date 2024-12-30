import Ajv, { JSONSchemaType } from "ajv";

const ajv = new Ajv({ allErrors: true });

type EncryptionRequest = {
    inputPath: string;
    outputPath: string;
    key: string;
};

const encryptionRequestSchema: JSONSchemaType<EncryptionRequest> = {
    type: "object",
    properties: {
        inputPath: { type: "string" },
        outputPath: { type: "string" },
        key: { type: "string" },
    },
    required: ["inputPath", "outputPath", "key"],
    additionalProperties: false,
};

export function validateEncryptionRequest(data: unknown): EncryptionRequest {
    const validate = ajv.compile(encryptionRequestSchema);
    if (!validate(data)) {
        throw new Error("Invalid JSON: " + ajv.errorsText(validate.errors));
    }
    return data as EncryptionRequest;
}

// Example Usage
if (require.main === module) {
    const testData = {
        inputPath: "/path/to/input",
        outputPath: "/path/to/output",
        key: "securekey",
    };

    try {
        const validatedData = validateEncryptionRequest(testData);
        console.log("Validation succeeded:", validatedData);
    } catch (error) {
        console.error("Validation failed:", error.message);
    }
}
