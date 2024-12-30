import { createLogger, format, transports } from "winston";

const logger = createLogger({
    level: "info",
    format: format.combine(
        format.timestamp(),
        format.errors({ stack: true }),
        format.json()
    ),
    transports: [
        new transports.Console({
            format: format.combine(format.colorize(), format.simple()),
        }),
        new transports.File({ filename: "shellmorph.log" }),
    ],
});

export function logInfo(message: string) {
    logger.info(message);
}

export function logError(error: Error) {
    logger.error(error);
}

export function logDebug(message: string) {
    logger.debug(message);
}

// Example Usage
if (require.main === module) {
    logInfo("Informational log example");
    logError(new Error("Error log example"));
    logDebug("Debug log example");
}
