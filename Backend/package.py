"""
崩坏：星穹铁道助手 社区版
原作者：雪影
修改：洋辰
打包
"""

import json
import os
import shutil
import sys
from pathlib import Path

if __name__ == "__main__":

    root_path = Path(sys.argv[0]).resolve().parent

    with (root_path / "version.json").open(mode="r", encoding="utf-8") as f:
        version = json.load(f)

    print("Packaging Python program...")

    os.system(
        "powershell -Command python -m nuitka --standalone --mingw64"
        " --windows-console-mode=force --windows-uac-admin"
        " --windows-icon-from-ico=resources\\SRAicon.ico"
        " --company-name='StarRailAssistant Team' --product-name=StarRailAssistant"
        f" --file-version={version['version'].split('-')[0]}"
        f" --product-version={version['version'].split('-')[0]}"
        " --file-description='StarRailAssistant Component'"
        " --copyright='Copyright © 2024 Shasnow'"
        " --assume-yes-for-downloads --output-filename=SRA-cli"
        " --remove-output main.py"
    )
    print("Python program packaging completed !")

    print("Start to copy resources ...")
    shutil.copytree(root_path / "resources", root_path / "main.dist/resources")
    shutil.copytree(root_path / "rapidocr_onnxruntime", root_path / "main.dist/rapidocr_onnxruntime")
    shutil.copytree(root_path / "tasks", root_path / "main.dist/tasks")
    os.makedirs(root_path / "main.dist/SRACore", exist_ok=True)
    shutil.copytree(root_path / "SRACore/i18n", root_path / "main.dist/SRACore/i18n")
    shutil.copy(root_path / "SRACore/config.toml", root_path / "main.dist/SRACore/config.toml")
    shutil.copy(root_path / "LICENSE", root_path / "main.dist/LICENSE")
    shutil.copy(root_path / "README.md", root_path / "main.dist/README.md")
    shutil.copy(root_path / "version.json", root_path / "main.dist/version.json")
    print("Resources copy completed !")

    print("Moving files to StarRailAssistant directory...")
    target_dir = root_path.parent / "StarRailAssistant"
    if target_dir.exists():
        shutil.rmtree(target_dir)
    shutil.move(root_path / "main.dist", target_dir)
    print("Files moved successfully !")

    print("Packaging completed !")

