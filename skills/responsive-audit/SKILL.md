---
name: responsive-audit
description: Use when auditing responsive design — 6 breakpoints, layout, touch targets, overflow, images
arguments:
  - name: target
    description: Page or component path to audit
    required: true
---

# Responsive Audit Workflow

## Step 1: Identify Target
- Read the target file(s)
- Understand the layout structure (flex, grid, absolute, etc.)
- List all breakpoint-dependent styles

## Step 2: Audit Each Breakpoint

### Mobile (375px)
- [ ] Single column layout where appropriate
- [ ] Touch targets min 44x44px
- [ ] No horizontal scroll
- [ ] Font size min 16px for body text
- [ ] Adequate spacing between interactive elements

### Small Mobile (320px)
- [ ] No content overflow or truncation breaking layout
- [ ] Navigation still usable
- [ ] Forms still fillable

### Tablet (768px)
- [ ] Layout adapts (2-column where appropriate)
- [ ] Images scale properly
- [ ] Navigation adapts (hamburger → tabs or vice versa)

### Desktop (1024px)
- [ ] Full layout utilizes space
- [ ] Max content width set (not stretching to infinity)
- [ ] Sidebar/aside content visible if applicable

### Wide (1280px)
- [ ] Content centered or max-width contained
- [ ] No excessive whitespace
- [ ] Images don't pixelate

### Ultra-wide (1920px+)
- [ ] Layout doesn't break
- [ ] Content doesn't stretch uncomfortably

## Step 3: Common Issues Check
- [ ] Images: `srcset` / responsive sizing, proper aspect ratios
- [ ] Typography: readable at all sizes, proper line lengths (45-75 chars)
- [ ] Spacing: consistent with design system tokens
- [ ] Overflow: no `overflow: hidden` hiding important content
- [ ] Z-index: modals/dropdowns work on all sizes
- [ ] Inputs: don't zoom on iOS (font-size >= 16px)

## Step 4: Issues Report
For each issue:
- Breakpoint where it occurs
- File and line number
- Screenshot description or CSS selector
- Suggested fix

Prioritize: broken layout > usability > polish
