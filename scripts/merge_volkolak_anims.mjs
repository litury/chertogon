#!/usr/bin/env node
/**
 * Объединяет 6 анимационных GLB Волколака в один файл.
 * БЕЗ prune/dedup — они ломают ссылки на ноды в single-scene моделях.
 */

import { NodeIO } from '@gltf-transform/core';

const ANIM_DIR = 'assets/models/enemies/volkolak_anims';
const OUTPUT = 'assets/models/enemies/volkolak_merged.glb';

// Порядок важен — индексы Animation0..Animation5 будут в этом порядке
const ANIMS = [
    { file: 'idle.glb',  name: 'idle' },
    { file: 'walk.glb',  name: 'walk' },
    { file: 'run.glb',   name: 'run' },
    { file: 'attack.glb', name: 'attack' },
    { file: 'hit.glb',    name: 'hit' },
    { file: 'death.glb',  name: 'death' },
];

const io = new NodeIO();

// Берём первый файл (idle) как базу — в нём mesh, skin, nodes
const baseDoc = await io.read(`${ANIM_DIR}/${ANIMS[0].file}`);
const baseRoot = baseDoc.getRoot();

// Переименовываем первую анимацию
const baseAnims = baseRoot.listAnimations();
if (baseAnims.length > 0) {
    baseAnims[0].setName(ANIMS[0].name);
    console.log(`Base: "${ANIMS[0].name}" (${baseAnims[0].listChannels().length} channels)`);
}

// Собираем маппинг имён нод
const nodeMap = new Map();
for (const node of baseRoot.listNodes()) {
    const name = node.getName();
    if (name && !nodeMap.has(name)) {
        nodeMap.set(name, node);
    }
}
console.log(`Base nodes: ${nodeMap.size}`);

// Мержим остальные 5 анимаций
for (let i = 1; i < ANIMS.length; i++) {
    const { file, name } = ANIMS[i];
    const srcDoc = await io.read(`${ANIM_DIR}/${file}`);
    const srcRoot = srcDoc.getRoot();

    for (const srcAnim of srcRoot.listAnimations()) {
        const newAnim = baseDoc.createAnimation(name);
        let ok = 0, skip = 0;

        for (const srcCh of srcAnim.listChannels()) {
            const srcSampler = srcCh.getSampler();
            const srcNode = srcCh.getTargetNode();
            const path = srcCh.getTargetPath();

            if (!srcSampler || !srcNode) { skip++; continue; }

            const target = nodeMap.get(srcNode.getName());
            if (!target) { skip++; continue; }

            const srcIn = srcSampler.getInput();
            const srcOut = srcSampler.getOutput();
            if (!srcIn || !srcOut) { skip++; continue; }

            const newIn = baseDoc.createAccessor()
                .setType(srcIn.getType())
                .setArray(srcIn.getArray().slice());
            const newOut = baseDoc.createAccessor()
                .setType(srcOut.getType())
                .setArray(srcOut.getArray().slice());

            const newSampler = baseDoc.createAnimationSampler()
                .setInput(newIn)
                .setOutput(newOut)
                .setInterpolation(srcSampler.getInterpolation());

            const newCh = baseDoc.createAnimationChannel()
                .setSampler(newSampler)
                .setTargetNode(target)
                .setTargetPath(path);

            newAnim.addSampler(newSampler);
            newAnim.addChannel(newCh);
            ok++;
        }
        console.log(`Merged: "${name}" (${ok} channels, ${skip} skipped)`);
    }
}

// Сохраняем БЕЗ prune/dedup
await io.write(OUTPUT, baseDoc);

// Верификация
const verify = await io.read(OUTPUT);
const finalAnims = verify.getRoot().listAnimations();
console.log(`\nOutput: ${OUTPUT}`);
console.log(`Animations: ${finalAnims.length}`);
for (const a of finalAnims) {
    console.log(`  - "${a.getName()}" (${a.listChannels().length} ch)`);
}
