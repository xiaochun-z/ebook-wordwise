# A tool for reading English books as an English learner
This is a tool which helps you read English books. As an English learner, when reading English novels, I found I needed to look up words in the dictionary frequently. This slows me down, and sometimes I feel frustrated since it takes me too long to finish reading a single book. 
Can I get the meaning of a new word without looking up the dictionary?

Then this tool comes out.

View more here if you cannot see screenshots. 如果你在中国，可能看不到截图，点击这个链接查看：
https://www.shenhe.org/en/article/ebook-wordwise.html

# Requirements
eBook Wordwise requires [calibre](https://calibre-ebook.com/download) to convert the books, please install calibre first to use this tool. For Windows, please consider adding calibre to your PATH in environment variables if you have installed calibre but this tool cannot detect it.

# Current support languages
English, Chinese, Arabic, German, Spanish, French, Hindi, Japanese, Korean, Portuguese, Russian, Thai, Ukrainian, Vietnamese.

# Support Platforms
* Windows
* Linux
* macOS

For macOS users, the provided release is not signed with an Apple Developer certificate. When you install and open the `.dmg`, macOS Gatekeeper may block it, stating that the app is "damaged and can't be opened" or "cannot be verified".

**How to run the unsigned app on macOS:**
1. Download the `.dmg` release and drag `ebook-wordwise` into your `Applications` folder.
2. Open the **Terminal** app (found in Applications > Utilities, or via Spotlight search).
3. Run the following command to remove the Apple quarantine attribute:
   ```bash
   sudo xattr -cr /Applications/ebook-wordwise.app
   ```
4. Enter your Mac login password when prompted (the characters won't show as you type).
5. Open the app from your Launchpad or Applications folder. It will now launch normally!

# Support formats
* epub
* mobi
* azw3
* pdf

The tool supports a lot more formats from calibre but they are not listed here.

# You can add/update/delete definitions in the .csv to customize your reading experience
You can customize your reading experience by editing the resource file. Click the **folder** icon on the left bar to open the resource directory, and you can add new words or phrases to the .csv file so the tool can recognize more words.

**Please always backup the updated .csv somewhere in case the app overwrites your changes when updating to a new version.**

You're welcome to contribute to the translation and help other book readers all around the world.

# Screenshots

![screenshot on Mac OS](examples/screenshot-macos.png)
![screenshot on Windows](examples/screenshot-windows.png)

## Screenshot on Amazon Kindle ⬇️

![Kindle](examples/kindle.png)

## Mac OS iBook

show definition as inlined text ⬇️

![MacOS Inline Style](examples/macos-inline.png)

show definition on the top ⬇️
![MacOS On Top Style](examples/macos-top.png)

# Install from source code
* Install Rust: https://www.rust-lang.org/tools/install
* Install NodeJS (npm is included): https://nodejs.org/en/download/package-manager
* Install Tauri prerequisites depending on your OS: https://v2.tauri.app/start/prerequisites/

Install dependencies and build the application:
```bash
# Install Javascript dependencies
npm install

# Run the app in development mode
npm run tauri dev

# Build the release application
npm run tauri build
```