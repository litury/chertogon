#!/usr/bin/env node
/**
 * Нормализует координаты волколака из Meshy-масштаба (~3545 единиц высоты)
 * в игровой масштаб (~1.4 единицы высоты).
 *
 * Домножает на SCALE:
 * - Все вершины (mesh positions)
 * - Все трансляции нод (node translations)
 * - Все translation-каналы анимаций
 *
 * Это позволяет использовать scale ~1.0 в игре вместо 0.0004,
 * устраняя z-fighting и проблемы с frustum culling.
 */

import { NodeIO } from '@gltf-transform/core';

const SCALE = 0.0004;
const ANIM_DIR = 'assets/models/enemies/volkolak_anims';

const FILES = ['idle.glb', 'walk.glb', 'run.glb', 'attack.glb', 'hit.glb', 'death.glb'];

const io = new NodeIO();

for (const file of FILES) {
    const path = `${ANIM_DIR}/${file}`;
    console.log(`\nProcessing: ${path}`);

    const doc = await io.read(path);
    const root = doc.getRoot();

    // 1. Масштабируем вершины мешей (position accessor)
    let meshCount = 0;
    for (const mesh of root.listMeshes()) {
        for (const prim of mesh.listPrimitives()) {
            const posAccessor = prim.getAttribute('POSITION');
            if (posAccessor) {
                const arr = posAccessor.getArray();
                for (let i = 0; i < arr.length; i++) {
                    arr[i] *= SCALE;
                }
                posAccessor.setArray(arr);
                meshCount++;
            }
        }
    }
    console.log(`  Meshes scaled: ${meshCount} primitives`);

    // 2. Масштабируем трансляции нод
    let nodeCount = 0;
    for (const node of root.listNodes()) {
        const t = node.getTranslation();
        if (t[0] !== 0 || t[1] !== 0 || t[2] !== 0) {
            node.setTranslation([t[0] * SCALE, t[1] * SCALE, t[2] * SCALE]);
            nodeCount++;
        }
    }
    console.log(`  Nodes translated: ${nodeCount}`);

    // 3. Масштабируем translation-каналы анимаций
    let animChannels = 0;
    for (const anim of root.listAnimations()) {
        for (const ch of anim.listChannels()) {
            if (ch.getTargetPath() === 'translation') {
                const sampler = ch.getSampler();
                if (sampler) {
                    const outAccessor = sampler.getOutput();
                    if (outAccessor) {
                        const arr = outAccessor.getArray();
                        for (let i = 0; i < arr.length; i++) {
                            arr[i] *= SCALE;
                        }
                        outAccessor.setArray(arr);
                        animChannels++;
                    }
                }
            }
        }
    }
    console.log(`  Animation translation channels scaled: ${animChannels}`);

    // 4. Масштабируем inverseBind matrices в скинах
    let skinCount = 0;
    for (const skin of root.listSkins()) {
        const ibmAccessor = skin.getInverseBindMatrices();
        if (ibmAccessor) {
            const arr = ibmAccessor.getArray();
            // В 4x4 матрице: translation в столбцах 12,13,14 (column-major)
            const matCount = arr.length / 16;
            for (let m = 0; m < matCount; m++) {
                const base = m * 16;
                arr[base + 12] *= SCALE;  // tx
                arr[base + 13] *= SCALE;  // ty
                arr[base + 14] *= SCALE;  // tz
            }
            ibmAccessor.setArray(arr);
            skinCount++;
        }
    }
    console.log(`  Skins scaled: ${skinCount}`);

    await io.write(path, doc);
    console.log(`  Written: ${path}`);
}

// Верификация: проверяем bbox первого файла
console.log('\n--- Verification ---');
const verDoc = await io.read(`${ANIM_DIR}/${FILES[0]}`);
const verRoot = verDoc.getRoot();
let minY = Infinity, maxY = -Infinity;
for (const mesh of verRoot.listMeshes()) {
    for (const prim of mesh.listPrimitives()) {
        const pos = prim.getAttribute('POSITION');
        if (pos) {
            const arr = pos.getArray();
            for (let i = 1; i < arr.length; i += 3) {
                if (arr[i] < minY) minY = arr[i];
                if (arr[i] > maxY) maxY = arr[i];
            }
        }
    }
}
console.log(`Model height (Y): ${minY.toFixed(4)} to ${maxY.toFixed(4)} = ${(maxY - minY).toFixed(4)} units`);
console.log('Done!');
