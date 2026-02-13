#!/usr/bin/env node
/**
 * Переносит анимацию из одного GLB в другой (при совпадающих скелетах).
 *
 * Использование:
 *   node scripts/merge_animation.mjs <target.glb> <source.glb> <output.glb>
 *
 * Пример:
 *   node scripts/merge_animation.mjs assets/models/enemies/upyr_merged.glb /tmp/meshy_hit_reaction.glb assets/models/enemies/upyr_merged.glb
 */

import { NodeIO } from '@gltf-transform/core';
import { dedup, prune } from '@gltf-transform/functions';

const [targetPath, sourcePath, outputPath] = process.argv.slice(2);

if (!targetPath || !sourcePath || !outputPath) {
    console.error('Usage: node merge_animation.mjs <target.glb> <source.glb> <output.glb>');
    process.exit(1);
}

const io = new NodeIO();

// Читаем оба файла
const targetDoc = await io.read(targetPath);
const sourceDoc = await io.read(sourcePath);

const targetRoot = targetDoc.getRoot();
const sourceRoot = sourceDoc.getRoot();

// Собираем маппинг имени ноды → нода в target
const targetNodeMap = new Map();
for (const node of targetRoot.listNodes()) {
    const name = node.getName();
    if (name) {
        // Берём первое совпадение (из первой сцены)
        if (!targetNodeMap.has(name)) {
            targetNodeMap.set(name, node);
        }
    }
}

console.log(`Target nodes: ${targetNodeMap.size}`);
console.log(`Source animations: ${sourceRoot.listAnimations().length}`);

let transferred = 0;

for (const sourceAnim of sourceRoot.listAnimations()) {
    const animName = sourceAnim.getName();
    console.log(`\nProcessing animation: "${animName}"`);
    console.log(`  Channels: ${sourceAnim.listChannels().length}`);
    console.log(`  Samplers: ${sourceAnim.listSamplers().length}`);

    // Создаём новую анимацию в target документе
    const newAnim = targetDoc.createAnimation(animName);

    let channelCount = 0;
    let skippedCount = 0;

    for (const srcChannel of sourceAnim.listChannels()) {
        const srcSampler = srcChannel.getSampler();
        const srcTargetNode = srcChannel.getTargetNode();
        const targetPath = srcChannel.getTargetPath();

        if (!srcTargetNode || !srcSampler) {
            skippedCount++;
            continue;
        }

        const nodeName = srcTargetNode.getName();
        const matchingNode = targetNodeMap.get(nodeName);

        if (!matchingNode) {
            console.log(`  SKIP: no matching node "${nodeName}" in target`);
            skippedCount++;
            continue;
        }

        // Копируем sampler данные
        const srcInput = srcSampler.getInput();
        const srcOutput = srcSampler.getOutput();

        if (!srcInput || !srcOutput) {
            skippedCount++;
            continue;
        }

        // Создаём accessor копии в target документе
        const newInput = targetDoc.createAccessor()
            .setType(srcInput.getType())
            .setArray(srcInput.getArray().slice());

        const newOutput = targetDoc.createAccessor()
            .setType(srcOutput.getType())
            .setArray(srcOutput.getArray().slice());

        // Создаём sampler
        const newSampler = targetDoc.createAnimationSampler()
            .setInput(newInput)
            .setOutput(newOutput)
            .setInterpolation(srcSampler.getInterpolation());

        // Создаём channel
        const newChannel = targetDoc.createAnimationChannel()
            .setSampler(newSampler)
            .setTargetNode(matchingNode)
            .setTargetPath(targetPath);

        newAnim.addSampler(newSampler);
        newAnim.addChannel(newChannel);
        channelCount++;
    }

    console.log(`  Transferred: ${channelCount} channels, Skipped: ${skippedCount}`);
    transferred++;
}

console.log(`\nTotal animations transferred: ${transferred}`);

// Очистка
await targetDoc.transform(prune(), dedup());

// Сохраняем
await io.write(outputPath, targetDoc);

// Верификация
const verifyDoc = await io.read(outputPath);
const anims = verifyDoc.getRoot().listAnimations();
console.log(`\nOutput file: ${outputPath}`);
console.log(`Total animations: ${anims.length}`);
for (const a of anims) {
    console.log(`  - "${a.getName()}" (${a.listChannels().length} channels)`);
}
