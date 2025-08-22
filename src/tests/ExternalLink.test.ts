/*
 * Copyright (c) 2025. Louis DERVELOY (https://github.com/LouisDerveloy/)
 * Licensed under the PolyForm Noncommercial License 1.0.0
 * SPDX-License-Identifier: PolyForm-Noncommercial-1.0.0
 * See the LICENSE file in the project root for details.
 */

import { mount } from '@vue/test-utils';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import ExternalLink from '@/components/ExternalLink.vue';
import { openUrl } from '@tauri-apps/plugin-opener';
import { writeText } from '@tauri-apps/plugin-clipboard-manager';

vi.mock('@tauri-apps/plugin-opener');
vi.mock('@tauri-apps/plugin-clipboard-manager');

describe('ExternalLink.vue', () => {
    beforeEach(() => vi.clearAllMocks());

    it('rend le slot et garde l’attribut href', () => {
        const wrapper = mount(ExternalLink, {
            props: { href: 'https://example.com' },
            slots: { default: 'Visit' },
        });
        const a = wrapper.get('a');
        expect(a.text()).toBe('Visit');
        expect(a.attributes('href')).toBe('https://example.com');
        expect(a.attributes('rel')).toBe('noopener');
    });

    it('click gauche ouvre via openUrl si http/https/mailto', async () => {
        const wrapper = mount(ExternalLink, {
            props: { href: 'https://example.com' },
            slots: { default: 'Go' },
        });
        await wrapper.get('a').trigger('click', { button: 0 });
        expect(openUrl).toHaveBeenCalledWith('https://example.com');
    });

    it('click milieu (auxclick) ouvre aussi', async () => {
        const wrapper = mount(ExternalLink, {
            props: { href: 'http://example.com' },
            slots: { default: 'Go' },
        });
        await wrapper.get('a').trigger('auxclick', { button: 1 });
        expect(openUrl).toHaveBeenCalledWith('http://example.com');
    });

    it('n’ouvre pas pour un schéma non supporté (ftp)', async () => {
        const wrapper = mount(ExternalLink, {
            props: { href: 'ftp://example.com' },
            slots: { default: 'Nope' },
        });
        await wrapper.get('a').trigger('click');
        expect(openUrl).not.toHaveBeenCalled();
    });

    it('copie le href si copybutton=true', async () => {
        const wrapper = mount(ExternalLink, {
            props: { href: 'mailto:user@example.com', copybutton: true },
            slots: { default: 'Mail' },
        });
        await wrapper.get('button').trigger('click');
        expect(writeText).toHaveBeenCalledWith('mailto:user@example.com');
    });
});
