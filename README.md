# LIV-PLYR

**Liv Plyr** is a cross-platform desktop application that supports strraming live IPTV channels via playlists.

> NOTE : You need to have **rust** and **node.js** with **npm** installed to develop on this project

---

### Push a feature/bug

1. Clone repo

```bash
# Clone full repo
git clone https://github.com/Sanjiv39/liv-plyr

# Clone only single branch, here dev
git clone --single-branch --branch dev https://github.com/Sanjiv39/liv-plyr

# Checkout to dir
cd liv-plyr
```

2. Install Node dependencies

```bash
npm install
# or
yarn add
```

3. Push changes to a branch, here ui

```bash
git add .
git commit -m "some ui changes"
git push origin ui
```

4. To merge your changes with the main development branch, raise a PR for `dev` branch from your feature branch.

---

### Development

- Run the frontend on **development** mode

```bash
npm run dev
```

- Run the complete application on **development** mode

```bash
npm run tauri dev
```

- Build the application (current platform)

```bash
# Builds frontend then rust backend for native os into respective executable file
npm run tauri build
```
