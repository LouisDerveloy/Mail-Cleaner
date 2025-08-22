/*
 * Copyright (c) 2025. Louis DERVELOY (https://github.com/LouisDerveloy/)
 * Licensed under the PolyForm Noncommercial License 1.0.0
 * SPDX-License-Identifier: PolyForm-Noncommercial-1.0.0
 * See the LICENSE file in the project root for details.
 */

import {vi} from "vitest";

// Mock minimal de @tauri-apps/api utilisé par les composants
vi.mock("@tauri-apps/api", () => {
    return {
        invoke: vi.fn(async (_cmd: string, _args?: Record<string, unknown>) => {
            // Par défaut, on renvoie "ok" pour simplifier, override dans les tests si besoin.
            return "ok";
        }),
    };
});

// Si tu utilises @tauri-apps/api/event:
vi.mock("@tauri-apps/api/event", () => {
    return {
        listen: vi.fn(async (_event: string, _cb: (payload: unknown) => void) => {
            return {
                unlisten: () => {
                }
            };
        }), emit: vi.fn(async () => {
        }),
    };
});

// Mock gsap de base
vi.mock('gsap', () => {
    const to = vi.fn();

    const context = vi.fn((cb?: () => void) => {
        const ctx = { revert: vi.fn() };
        (globalThis as any).__gsapLastContext = ctx; // Pour les assertions d’unmount
        if (cb) cb();
        return ctx;
    });

    const timeline = vi.fn(() => ({
        to: vi.fn().mockReturnThis(),
        play: vi.fn(),
        reverse: vi.fn(),
    }));

    const gsapMock = {
        to,
        context,           // <— c’est un vi.fn maintenant
        timeline,
        registerPlugin: vi.fn(),
    };

    // Supporte import default et nommés
    return { default: gsapMock, ...gsapMock };
});

// Mock du plugin Observer (importé depuis 'gsap/Observer')
vi.mock('gsap/Observer', () => {
    return { Observer: { create: vi.fn() } };
});

// Mock Tauri plugins utilisés par ExternalLink
vi.mock('@tauri-apps/plugin-opener', () => ({
    openUrl: vi.fn(async (_url: string) => {}),
}));
vi.mock('@tauri-apps/plugin-clipboard-manager', () => ({
    writeText: vi.fn(async (_text: string) => {}),
}));

// Mock assets
vi.mock("/\.png$/i", () => ({ default: 'mock.png' }), { virtual: true });
vi.mock("/\.svg$/i", () => ({ default: 'mock.svg' }), { virtual: true });

// jsdom: définir getComputedStyle si nécessaire (certaines lib d’anim le lisent)
if (!(window as any).getComputedStyle) {
    (window as any).getComputedStyle = () => ({
        getPropertyValue: () => '',
    });
}
