/*
 * Copyright (c) 2025. Louis DERVELOY (https://github.com/LouisDerveloy/)
 * Licensed under the PolyForm Noncommercial License 1.0.0
 * SPDX-License-Identifier: PolyForm-Noncommercial-1.0.0
 * See the LICENSE file in the project root for details.
 */

import { message } from "@tauri-apps/plugin-dialog";

// This mirrors the FailureType enum in src-tauri/src/utils.rs
// when serialized by serde.
type FailureType =
  { ImapConnectionError: string }
  | 'FailedToLockState'
  | 'NotConnected'
  | { UnknownError: string }
  | 'IDDidntMatch'
  | { ChannelError: String };

function getErrorMessage(error: unknown): string {
    // Type assertion to treat the unknown error as a potential FailureType
    const failure = error as FailureType;

    if (typeof failure === 'string') {
        switch (failure) {
            case 'FailedToLockState':
                return "An internal application error occurred. Please try restarting the application if the problem persists.";
            case 'NotConnected':
                return "You are not connected to a mail server. Please go to the connection page.";
            case 'IDDidntMatch':
                return "An internal application error occurred due to an ID mismatch. Please try the operation again.";
        }
    }

    if (typeof failure === 'object' && failure !== null) {
        if ('ImapConnectionError' in failure && typeof (failure as any).ImapConnectionError === 'string') {
            const imapError = (failure as { ImapConnectionError: string }).ImapConnectionError.toLowerCase();
            if (imapError.includes("authentication") || imapError.includes("credentials")) {
                return "Authentication failed. Please double-check your email/password or token. For some providers like Gmail, you may need to use an 'App Password'.";
            }
            if (imapError.includes("timed out")) {
                return "The connection to the mail server timed out. Check your internet connection and try again.";
            }
            // General connection error.
            return "Could not connect to the mail server. Please check the server address, port, and your internet connection.";
        }
        if ('UnknownError' in failure && typeof (failure as any).UnknownError === 'string') {
            return `An unknown error occurred: ${(failure as { UnknownError: string }).UnknownError}`;
        }
        if ('ChannelError' in failure && typeof (failure as any).ChannelError === 'string') {
            return `An error occurred in channel communication: ${(failure as { ChannelError: string }).ChannelError}`;
        }
    }
    
    console.error("handleError received an unrecognized error format:", error);
    return "An unexpected error occurred. Please check the application logs for more details.";
}


export async function handleError(error: unknown) {
  const userFriendlyMessage = getErrorMessage(error);

  await message(userFriendlyMessage, {
    title: "An Error Occurred",
    kind: "error",
  });
} 