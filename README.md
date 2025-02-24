# Libreoffice Scientific Dictionaries

A collection of different dictionaries for scientific fields and languages.

The purpose is to have specialized dictionaries for various fields, maintained by the community.

### Use

Download a dictionary file into your custom "wordbook" folder:

- Linux, native: `~/.config/libreoffice/4/user/wordbook/`
- Linux, Flatpak: `~/.var/app/org.libreoffice.LibreOffice/config/libreoffice/4/user/wordbook/`
- Linux, Snap: `~/snap/libreoffice/current/.config/libreoffice/4/user/wordbook/`
- Windows: `C:\Users\<YourUsername>\AppData\Roaming\LibreOffice\4\user\wordbook\`
- macOS: `~/Library/Application Support/LibreOffice/4/user/wordbook/`

For example, [the list of common FOSS tools](https://raw.githubusercontent.com/boredsquirrel/Libreoffice-Scientific-Dictionaries/refs/heads/main/dictionaries/computer-science/0-tools-distros-licenses.dic).

Go to a file, click `Raw` and use `Ctrl+S` to save it.

### Overview
You find the dictionary categories under `dictionaries`.

In there may be dictionary files that are generic and/or language-independent, e.g. `0-atoms-molecules.dic`. These get a `0` to place them at the front.

Other dictionaries are language dependent, their naming should follow `countrycode-category.dic`, e.g. `en-biology.dic`.

### File structure
All dictionaries have this header, example in german:

```
OOoUserDict1
lang: de-DE
type: positive
---
```

The entries are alphabetically sorted and should not be duplicated.


The tool `dictbuilder` can do the formatting, so you can just dump your dictionary list in there.

### Contribute

1. [Fork this repo](https://github.com/boredsquirrel/Libreoffice-Scientific-Dictionaries/fork)
2. Copy your related words to the end of a dictionary file
3. [Open a pull request](https://github.com/boredsquirrel/Libreoffice-Scientific-Dictionaries/compare)

You can create a custom dictionary in Libreoffice, to sort your edits:

1. Menubar -> `Tools`
2. `Options`
3. `Languages and Locales`
4. `Writing Aids`
5. `New...`

Now you can add words to that specific wordbook using the right-click menu.
