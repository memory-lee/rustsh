#!/bin/bash

echo "=== Test basic ls ==="
echo -e "ls\nexit" | cargo run

echo ""
echo "=== Test ls with args ==="
echo -e "ls -l\nexit" | cargo run

echo ""
echo "=== Test cd into directory ==="
echo -e "cd src\nls\nexit" | cargo run

echo ""
echo "=== Test cd back ==="
echo -e "cd src\ncd ..\nls\nexit" | cargo run

echo ""
echo "=== Test cd to root ==="
echo -e "cd /\nls\nexit" | cargo run

echo ""
echo "=== Test cd with no args (should error) ==="
echo -e "cd\nexit" | cargo run

echo ""
echo "=== Test cd with too many args (should error) ==="
echo -e "cd a b\nexit" | cargo run

echo ""
echo "=== Test cd to nonexistent dir (should error) ==="
echo -e "cd nonexistent_folder\nexit" | cargo run

echo ""
echo "=== Test exit with args (should error) ==="
echo -e "exit abc\nexit" | cargo run

echo ""
echo "=== Test absolute path ==="
echo -e "/bin/ls\nexit" | cargo run

echo ""
echo "=== Test invalid program ==="
echo -e "nonexistent_command\nexit" | cargo run

echo ""
echo "=== Test empty input ==="
echo -e "\n\n\nexit" | cargo run