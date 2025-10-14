# 应用图标制作指南

## 快速创建图标

### 方法 1: 使用在线工具 (推荐)

#### Icon Kitchen (最简单)

1. 访问 https://icon.kitchen/
2. 点击 "Upload Image"
3. 上传你的 Logo (建议 1024x1024 PNG,透明背景)
4. 下载所有平台的图标
5. 解压到 `src-tauri/icons/` 目录

#### RealFaviconGenerator

1. 访问 https://realfavicongenerator.net/
2. 上传主图标
3. 选择 Desktop App 选项
4. 生成并下载图标包
5. 提取需要的尺寸

### 方法 2: 使用 Figma (专业)

**设计建议**:
- 尺寸: 1024x1024 像素
- 格式: PNG,透明背景
- 内容区域: 居中 800x800 (留白边距)
- 风格: 简洁、识别度高

**图标设计思路**:

```
企业微信多开工具
━━━━━━━━━━━━━━━
主题: 企业微信 + 多窗口概念

方案 1: 微信图标 + 多层叠加效果
  [W] [W]
    [W]

方案 2: 微信图标 + x2/x3 标记
  [W] x3

方案 3: 多个小微信图标组合
  [W][W]
  [W][W]
```

**推荐配色**:
- 主色: #07C160 (微信绿)
- 辅色: #10AEFF (企业微信蓝)
- 背景: 白色或渐变

### 方法 3: AI 生成 (创意)

使用 AI 工具生成:
- DALL-E: "app icon, corporate wechat, multiple windows, minimalist, flat design"
- Midjourney: "mobile app icon for enterprise wechat multi-instance launcher --style minimal"
- Stable Diffusion: "app icon, wechat logo with multiple layers, modern, clean"

## 需要的文件清单

在 `src-tauri/icons/` 目录创建:

```
src-tauri/icons/
├── 32x32.png          # 任务栏图标
├── 128x128.png        # 通知图标
├── 128x128@2x.png     # 高分屏通知图标
├── icon.ico           # Windows 图标 (多尺寸)
└── icon.icns          # macOS 图标 (可选)
```

## 使用 ImageMagick 批量生成

如果你有一个主图标 (1024x1024):

```bash
# 安装 ImageMagick
# Windows: choco install imagemagick
# macOS: brew install imagemagick

# 生成不同尺寸
convert main-icon.png -resize 32x32 32x32.png
convert main-icon.png -resize 128x128 128x128.png
convert main-icon.png -resize 256x256 128x128@2x.png

# 生成 ICO (Windows)
convert main-icon.png -define icon:auto-resize=256,128,96,64,48,32,16 icon.ico

# 生成 ICNS (macOS)
# 需要创建 iconset 目录并生成多个尺寸
mkdir icon.iconset
sips -z 16 16     main-icon.png --out icon.iconset/icon_16x16.png
sips -z 32 32     main-icon.png --out icon.iconset/icon_16x16@2x.png
sips -z 32 32     main-icon.png --out icon.iconset/icon_32x32.png
sips -z 64 64     main-icon.png --out icon.iconset/icon_32x32@2x.png
sips -z 128 128   main-icon.png --out icon.iconset/icon_128x128.png
sips -z 256 256   main-icon.png --out icon.iconset/icon_128x128@2x.png
sips -z 256 256   main-icon.png --out icon.iconset/icon_256x256.png
sips -z 512 512   main-icon.png --out icon.iconset/icon_256x256@2x.png
sips -z 512 512   main-icon.png --out icon.iconset/icon_512x512.png
sips -z 1024 1024 main-icon.png --out icon.iconset/icon_512x512@2x.png
iconutil -c icns icon.iconset
```

## 临时图标方案

如果暂时没有专业图标,可以使用:

### 简易文字图标

使用在线文字 Logo 生成器:
1. https://www.designevo.com/
2. 输入文字 "企微多开" 或 "WW"
3. 选择合适的样式
4. 导出 1024x1024 PNG

### 使用 Emoji

```python
# 使用 Python 生成简单图标
from PIL import Image, ImageDraw, ImageFont

# 创建 1024x1024 图标
size = 1024
img = Image.new('RGBA', (size, size), (7, 193, 96, 255))
draw = ImageDraw.Draw(img)

# 添加文字
font = ImageFont.truetype("arial.ttf", 400)
draw.text((size/2, size/2), "W×3", fill='white', font=font, anchor='mm')

# 保存
img.save('main-icon.png')
```

### 使用开源图标库

- https://icons8.com/ - 搜索 "wechat" 或 "messenger"
- https://www.flaticon.com/ - 搜索 "chat multiple"
- https://iconscout.com/ - 搜索 "app window"

**注意版权**: 使用开源图标需遵守相应许可证

## 设计规范

### Windows 图标规范

- **尺寸**: 16x16, 32x32, 48x48, 256x256
- **格式**: ICO (包含多个尺寸)
- **背景**: 可以有背景色
- **边距**: 保持适当的安全区域

### 设计要点

1. **简洁**: 图标要在小尺寸下清晰可辨
2. **识别度**: 快速传达应用功能
3. **一致性**: 与应用主题风格统一
4. **对比度**: 确保在深色/浅色背景下都清晰

## 测试图标

### Windows 测试

```powershell
# 构建后查看图标
npm run tauri:build

# 检查以下位置的图标显示:
# 1. 安装程序图标
# 2. 开始菜单图标
# 3. 任务栏图标
# 4. Alt+Tab 切换图标
# 5. 桌面快捷方式图标
```

### 图标质量检查

- [ ] 16x16 清晰可辨
- [ ] 32x32 细节完整
- [ ] 128x128 精致美观
- [ ] ICO 文件包含多个尺寸
- [ ] 透明背景正确
- [ ] 颜色在深色/浅色背景下都清晰

## 更新图标后

```powershell
# 1. 清理旧构建
npm run tauri:build -- --clean

# 2. 重新构建
npm run tauri:build

# 3. 重新安装测试
```

## 示例图标下载

临时使用这个简单图标:

创建 `scripts/generate_temp_icon.py`:

```python
from PIL import Image, ImageDraw, ImageFont

def create_icon():
    # 创建 1024x1024 图标
    size = 1024
    img = Image.new('RGBA', (size, size), (0, 0, 0, 0))
    draw = ImageDraw.Draw(img)

    # 绘制圆形背景
    draw.ellipse([50, 50, 974, 974], fill=(7, 193, 96, 255))

    # 添加文字 "多开"
    try:
        font = ImageFont.truetype("arial.ttf", 350)
    except:
        font = ImageFont.load_default()

    draw.text((512, 512), "W×3", fill='white', font=font, anchor='mm')

    # 保存主图标
    img.save('src-tauri/icons/main-icon.png')

    # 生成各尺寸
    img.resize((32, 32), Image.LANCZOS).save('src-tauri/icons/32x32.png')
    img.resize((128, 128), Image.LANCZOS).save('src-tauri/icons/128x128.png')
    img.resize((256, 256), Image.LANCZOS).save('src-tauri/icons/128x128@2x.png')

    print("图标已生成!")

if __name__ == '__main__':
    create_icon()
```

运行:
```bash
pip install pillow
python scripts/generate_temp_icon.py
```

---

图标准备好后,直接运行 `npm run tauri:build` 即可构建带图标的安装包!
