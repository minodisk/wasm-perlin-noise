const wait = delay =>
  new Promise(resolve => {
    setTimeout(resolve, delay);
  });

const next = () => new Promise(resolve => requestAnimationFrame(resolve));

const main = async () => {
  console.log("loading wasm");
  const { instance, module } = await fetchWASM("lib.wasm");
  console.log("loaded wasm");
  const { alloc, dealloc, perlin_noise } = instance.exports;

  const $canvas = document.querySelector("canvas");
  const $width = document.querySelector("#width");
  const $height = document.querySelector("#height");
  const $time = document.querySelector("#time");
  const $octaves = document.querySelector("#octaves");
  const $persistence = document.querySelector("#persistence");
  const $repeatX = document.querySelector("#repeatX");
  const $repeatY = document.querySelector("#repeatY");
  const $repeatZ = document.querySelector("#repeatZ");

  const ctx = $canvas.getContext("2d");

  const ptr = alloc(50 * 50 * 4);

  const render = () => {
    const width = parseInt($width.value, 10);
    const height = parseInt($height.value, 10);
    const time = parseInt($time.value, 10);
    const octaves = parseInt($octaves.value, 10);
    const persistence = parseInt($persistence.value, 10);
    const repeatX = parseInt($repeatX.value, 10);
    const repeatY = parseInt($repeatY.value, 10);
    const repeatZ = parseInt($repeatZ.value, 10);

    // console.log(
    //   width,
    //   height,
    //   time,
    //   octaves,
    //   persistence,
    //   repeatX,
    //   repeatY,
    //   repeatZ
    // );

    if ($canvas.width != width) {
      $canvas.width = width;
    }
    if ($canvas.height != height) {
      $canvas.height = height;
    }

    const size = width * height * 4;
    perlin_noise(
      ptr,
      width,
      height,
      time,
      octaves,
      persistence,
      repeatX,
      repeatY,
      repeatZ,
      5,
      20,
      6
    );
    const data = new Uint8ClampedArray(
      instance.exports.memory.buffer,
      ptr,
      size
    );
    // console.log(data);
    const image = new ImageData(data, width, height);
    ctx.putImageData(image, 0, 0);
  };

  [
    $width,
    $height,
    $time,
    $octaves,
    $persistence,
    $repeatX,
    $repeatY,
    $repeatZ
  ].forEach($input => $input.addEventListener("change", render));

  for (;;) {
    $time.value++;
    render();
    await next();
  }

  dealloc(ptr, size);
};

const fetchWASM = async path => {
  const res = await fetch(path);
  const buf = await res.arrayBuffer();
  const memory = new WebAssembly.Memory({ initial: 256, maximum: 256 });
  return WebAssembly.instantiate(buf, {
    env: {
      fmod: (a, b) => Number((a - Math.floor(a / b) * b).toPrecision(8))
    }
  });
};

(async () => {
  try {
    await main();
  } catch (err) {
    console.error(err);
  }
})();
