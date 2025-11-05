function build_qrgen() {
  echo \[BUILD\] Building qrgen...
  cd qrgen
  cargo install wasm-pack
  if ! wasm-pack build --target web --release; then
    echo "Build qrgen failed."
    exit 1
  fi

  echo \[BUILD\] Editing generated build...
  sed -i "s|        module_or_path = new URL('qrgen_wasm_bg.wasm', import.meta.url);|        module_or_path = new URL('/qrgen/qrgen_wasm_bg.wasm', import.meta.url);|g" pkg/qrgen_wasm.js
  edit_status=$?

  if [ $edit_status -eq 1 ]; then
    echo "Configure qrgen failed."
    exit 1
  fi

  echo \[BUILD\] Moving wasm build to web\'s /public directory...
  mkdir -p ../web/public/qrgen && mv pkg/qrgen_wasm_bg.wasm ../web/public/qrgen
  move_status=$?

  if [ $move_status -eq 1 ]; then
    echo "Configure qrgen failed."
    exit 1
  fi
}

function build_server() {
  echo \[BUILD\] Building server...
  cd ../server
  if ! cargo build --release; then
    echo "Build server failed."
    exit 1
  fi
}

function build_web() {
  echo \[BUILD\] Building web...
  cd ../web
  if ! yarn & yarn build; then
    echo "Build web failed."
    exit 1
  fi
}

if [ $1 == "qrgen" ]; then
  build_qrgen
fi
if [ $1 == "server" ]; then
  build_server
fi
if [ $1 == "web" ]; then
  build_web
fi
if [ $1 == "all" ]; then
  build_qrgen
  build_server
  build_web
fi

