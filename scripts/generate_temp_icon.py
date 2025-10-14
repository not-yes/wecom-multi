#!/usr/bin/env python3
"""
临时图标生成脚本
快速生成简单的应用图标用于开发测试
"""

import os
from pathlib import Path

try:
    from PIL import Image, ImageDraw, ImageFont
except ImportError:
    print("请先安装 Pillow: pip install pillow")
    exit(1)


def create_temp_icons():
    """创建临时图标"""

    # 图标输出目录
    icon_dir = Path(__file__).parent.parent / "src-tauri" / "icons"
    icon_dir.mkdir(parents=True, exist_ok=True)

    print(f"图标输出目录: {icon_dir}")

    # 创建主图标 (1024x1024)
    size = 1024
    img = Image.new('RGBA', (size, size), (0, 0, 0, 0))
    draw = ImageDraw.Draw(img)

    # 绘制渐变背景圆形
    # 企业微信蓝绿色
    colors = [
        (16, 174, 255),  # 企业微信蓝
        (7, 193, 96),    # 微信绿
    ]

    # 简化版: 使用单色背景
    bg_color = (7, 193, 96, 255)  # 微信绿

    # 绘制圆形背景
    padding = 80
    draw.ellipse(
        [padding, padding, size-padding, size-padding],
        fill=bg_color
    )

    # 尝试加载字体
    font_size = 350
    try:
        # Windows
        font_path = "C:/Windows/Fonts/arial.ttf"
        if os.path.exists(font_path):
            font = ImageFont.truetype(font_path, font_size)
        else:
            # macOS
            font_path = "/System/Library/Fonts/Helvetica.ttc"
            if os.path.exists(font_path):
                font = ImageFont.truetype(font_path, font_size)
            else:
                font = ImageFont.load_default()
                print("警告: 使用默认字体,显示效果可能不佳")
    except:
        font = ImageFont.load_default()
        print("警告: 使用默认字体,显示效果可能不佳")

    # 绘制文字 "W×3"
    text = "W×3"

    # 获取文字边界框
    bbox = draw.textbbox((0, 0), text, font=font)
    text_width = bbox[2] - bbox[0]
    text_height = bbox[3] - bbox[1]

    # 居中绘制
    x = (size - text_width) / 2 - bbox[0]
    y = (size - text_height) / 2 - bbox[1]

    draw.text((x, y), text, fill='white', font=font)

    # 保存主图标
    main_icon_path = icon_dir / "main-icon.png"
    img.save(main_icon_path)
    print(f"✓ 生成主图标: {main_icon_path}")

    # 生成各种尺寸
    sizes = [
        (32, "32x32.png"),
        (128, "128x128.png"),
        (256, "128x128@2x.png"),
    ]

    for size, filename in sizes:
        resized = img.resize((size, size), Image.Resampling.LANCZOS)
        output_path = icon_dir / filename
        resized.save(output_path)
        print(f"✓ 生成 {filename}: {output_path}")

    # 生成 ICO 文件 (Windows)
    try:
        ico_sizes = [(16, 16), (32, 32), (48, 48), (256, 256)]
        ico_images = []
        for size in ico_sizes:
            ico_images.append(img.resize(size, Image.Resampling.LANCZOS))

        ico_path = icon_dir / "icon.ico"
        ico_images[0].save(
            ico_path,
            format='ICO',
            sizes=ico_sizes,
            append_images=ico_images[1:]
        )
        print(f"✓ 生成 Windows 图标: {ico_path}")
    except Exception as e:
        print(f"⚠ ICO 生成失败: {e}")

    print("\n图标生成完成!")
    print("\n下一步:")
    print("1. 运行 'npm run tauri:build' 构建应用")
    print("2. 或者替换为专业设计的图标")
    print("\n专业图标制作指南: docs/ICON_GUIDE.md")


if __name__ == '__main__':
    create_temp_icons()
