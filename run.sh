#!/bin/bash

(cd companion && npm run build)
(cd examples/demo && trunk serve)