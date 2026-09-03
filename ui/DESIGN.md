---
system: broadcast-control-surface
register: product
theme: dark
tokens:
  canvas: "#090d12"
  sidebar: "#0d131a"
  content: "#111820"
  panel: "#151e27"
  accent: "#57c6ce"
  warning: "#e4a94c"
  danger: "#ef5a67"
  success: "#56c991"
  radius_panel: 10px
  radius_control: 6px
  motion_fast: 180ms
---

# GoXLR Broadcast Control Surface

## Overview

The interface is a dark broadcast console for streamers and audio operators working in dim rooms. It should feel like reliable studio equipment: compact, calm, legible, and immediately responsive.

**The Hardware First Rule.** Mixer state and device health outrank decoration on every screen.

## Colors

Two neutral layers separate navigation (`#0d131a`) from working content (`#111820`). Cyan (`#57c6ce`) means active, selected, or directly adjustable. Amber is reserved for warnings, red for clipping and destructive actions, and green for confirmed health.

**The Signal Rule.** Accent color communicates signal or action; it is never ambient decoration.

Audit test: if cyan can be removed without losing state information, that use is decorative and must be removed.

## Typography

League Mono carries status, numeric values, and compact labels. League Mono Condensed carries control names and navigation. UI labels remain at readable weights and never use all-caps paragraph text.

**The Read-at-a-Glance Rule.** A control name and its live value must be distinguishable without leaning toward the screen.

## Elevation

Panels use a subtle 1px cool border and a broad, low-opacity shadow. Controls use surface contrast instead of stacked shadows. Hardware previews may sit one layer deeper to resemble a recessed console bay.

**The Two-Layer Rule.** Every view uses one navigation surface and one working surface; additional boxes require functional grouping.

## Components

Navigation targets are at least 44px high. Selected navigation uses a quiet cyan tint and a signal dot. Faders keep a visible track, a high-contrast thumb, and an editable numeric value. Panels use 10px corners; controls use 6px corners.

**The Familiar Controls Rule.** Sliders, tabs, buttons, dialogs, and keyboard behavior remain standard and predictable.

## Do's and Don'ts

- Do keep focus rings visible and cyan.
- Do preserve keyboard navigation, ARIA relationships, device commands, and live state updates.
- Do make the mixer horizontally scrollable on narrow screens instead of shrinking faders below usable size.
- Do respect `prefers-reduced-motion`.
- Do not use glassmorphism, decorative neon, gradient text, or perpetual animation.
- Do not use cyan for inactive decoration.
- Do not hide errors or clipping behind color alone.
- Do not change the established GoXLR control terminology.
