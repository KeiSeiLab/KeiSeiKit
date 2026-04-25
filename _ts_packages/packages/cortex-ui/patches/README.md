# patches/

## pixi-live2d-display.patch

Copied verbatim from AIRI (https://github.com/moeru-ai/airi), MIT License.

### What it does

Both `FileLoader` and `ZipLoader` in `pixi-live2d-display@0.4.0` pick the first
settings file ending in `model.json` / `model3.json`. Newer Live2D authoring
tools emit `items_pinned_to_model.json` as a side-car, which accidentally
matches the suffix and derails the loader. The patch filters it out.

### Is it applied?

Not automatically. This project uses vanilla `npm install`, so we cannot
pre-apply patches the way `pnpm` + `patchedDependencies` or
`patch-package` would. Two options if you need it:

1. **Manual**, once per clone, from `_ts_packages/`:
   ```sh
   patch -p1 -d node_modules/pixi-live2d-display < packages/cortex-ui/patches/pixi-live2d-display.patch
   ```
2. **Automated**, wire up `patch-package`:
   ```sh
   npm i -D patch-package
   # add "postinstall": "patch-package" to root package.json
   ```
   (This would touch `_ts_packages/package.json` which is out of scope for
   the current wave; leaving as a follow-up.)

### Do we need it here?

Not for the bundled Haru model — `Live2DPet.svelte` loads the explicit
`model3.json` URL straight from `/public/live2d-models/…`, bypassing the
filename-scanning code paths the patch touches. The patch becomes relevant
only when (a) user-uploaded ZIPs are supported or (b) a model folder
contains an `items_pinned_to_model.json` sidecar.
