<script setup lang="ts">
import { emit } from "@tauri-apps/api/event";
import { convertFileSrc } from "@tauri-apps/api/tauri";
import { appWindow } from "@tauri-apps/api/window";
import Konva from "konva";
import { Shortcuts } from "shortcuts";
import { computed, onMounted, ref } from "vue";
import { useRoute } from "vue-router";

const route = useRoute();
let container = ref(null);
let imageUrl = ref("");
let schemaGlobal = ref("");
// eslint-disable-next-line @typescript-eslint/no-unused-vars
let stage: any, layer: any, rect: any, transformer: any;
let isDown = false;
let rectOption: any = {};

const shortcuts = new Shortcuts();

shortcuts.add([
  {
    shortcut: "Esc",
    handler: () => {
      if (rect) {
        rect = null;
        isDown = false;
        layer.destroyChildren();
        layer.draw();
      } else {
        emit("window-esc", appWindow.label);
      }
    },
  },
]);

const imageUrla = computed(() => {
  return convertFileSrc(imageUrl.value, schemaGlobal.value);
});

onMounted(async () => {
  const { image, schema } = route.query as any;
  imageUrl.value = image;
  schemaGlobal.value = schema;

  stage = createStage();
  layer = createLayer(stage);
});

const createStage = () => {
  return new Konva.Stage({
    container: container.value as any,
    width: window.innerWidth,
    height: window.innerHeight,
  });
};

const createLayer = (stage: Konva.Stage) => {
  let layer = new Konva.Layer();
  stage.add(layer);
  layer.draw();
  return layer;
};

function createRect(
  layer: Konva.Layer,
  x: number,
  y: number,
  w = 0,
  h = 0,
  opacity = 0,
  draggable = false
) {
  const width = w,
    height = h;
  let rect = new Konva.Rect({
    x,
    y,
    width,
    height,
    fill: `rgba(255,0,0,${opacity})`,
    name: "rect",
    draggable,
  });
  layer.add(rect);
  return rect;
}

function onMouseDown(e: any) {
  if (rect || isDown) return;
  isDown = true;
  const { pageX, pageY } = e;
  rectOption.x = pageX || 0;
  rectOption.y = pageY || 0;
  // rect = createRect(layer, pageX, pageY, 0, 0, 0.25, false);
  // rect.draw();
}

function onMouseMove(e: any) {
  if (!isDown) return;
  const { pageX, pageY } = e;
  let w = pageX - rectOption.x;
  let h = pageY - rectOption.y;
  if (rect) {
    rect.remove();
  }
  rect = createRect(layer, rectOption.x, rectOption.y, w, h, 0.25, false);
  rect.draw();
}

function onMouseUp(e: any) {
  if (!isDown) return;
  isDown = false;
  const { pageX, pageY } = e;

  let w = pageX - rectOption.x;
  let h = pageY - rectOption.y;
  // 如果没有移动不画框
  if (w === 0 && h === 0) {
    return;
  }

  if (rect) {
    rect.remove();
  }
  rect = createRect(layer, rectOption.x, rectOption.y, w, h, 0.5, true);
  rect.draw();
  //
  transformer = createTransformer(rect);
}

const createTransformer = (rect: Konva.Rect) => {
  var tr = new Konva.Transformer();
  layer.add(tr);
  tr.nodes([rect]);
  layer.draw();
  rect.on("dblclick", () => {
    handleCut();
  });
};

const newImage = (src: string) => {
  return new Promise((resolve, reject) => {
    let img = new Image();
    img.crossOrigin = "anonymous";
    img.onload = () => resolve(img);
    img.onerror = reject;
    img.src = src;
  });
};

const getCutImage = async (info: any) => {
  const { x, y, width, height } = info;
  let canvas = document.createElement("canvas");
  let ctx: any = canvas.getContext("2d");
  canvas.width = ctx.width = width;
  canvas.height = ctx.height = height;
  let img = await newImage(imageUrla.value);
  ctx.drawImage(img, -x, -y, window.innerWidth, window.innerHeight);

  return canvas.toDataURL("image/png");
};

const handleCut = async () => {
  const { width, height, x, y, scaleX = 1, scaleY = 1 } = rect.attrs;
  let _x = width > 0 ? x : x + width * scaleX;
  let _y = height > 0 ? y : y + height * scaleY;
  let pic = await getCutImage({
    x: _x,
    y: _y,
    width: Math.abs(width) * scaleX,
    height: Math.abs(height) * scaleY,
  });
  // 目的是发给主窗体页面让其接收到这个图片
  await emit("screenshotOk", pic);
};
</script>

<template>
  <div>
    <div
      class="sc-souce-container"
      :style="'z-index: 1;background-image:url(' + imageUrla + ')'"
    ></div>
    <div
      class="sc-souce-container sc-source-border"
      ref="container"
      style="z-index: 2"
      @mousedown="onMouseDown"
      @mousemove="onMouseMove"
      @mouseup="onMouseUp"
    ></div>
  </div>
</template>

<style scoped>
.sc-souce-container {
  width: 100%;
  height: 100vh;
  overflow: hidden;
  background-color: transparent;
  background-size: 100% 100%;
  background-repeat: no-repeat;
  position: absolute;
}

.sc-source-border {
  border-style: dashed;
  border-width: 2px;
  box-sizing: border-box;
  border-color: #24c8db;
}

body {
  margin: 0 !important;
}
</style>
