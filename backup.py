#!/usr/bin/env python3

import tkinter as tk
from tkinter import ttk
from tkinter import filedialog
from PIL import Image, ImageTk
import os

DEFAULT_TITLE = 'Backup'
DEFAULT_WIDTH = 640
DEFAULT_HEIGHT = 480

def load_image(filename):
    img = Image.open(os.path.join('icons', filename))
    img = img.resize( (16,16))
    img2 = Image.new('RGBA', (24, 16))
    img2.paste(img)
    return ImageTk.PhotoImage(img2)

def main():
    root = tk.Tk()
    root.title(DEFAULT_TITLE)
    root.iconbitmap(os.path.join('icons','logo.ico'))
    root.geometry(f'{DEFAULT_WIDTH}x{DEFAULT_HEIGHT}')
    root.minsize(DEFAULT_WIDTH, DEFAULT_HEIGHT)

    file_image = load_image('file.png')
    folder_image = load_image('folder.png')
    remove_image = load_image('remove.png')
    start_image = load_image('start.png')
    exchange_image = load_image('exchange.png')
    update_image = load_image('update.png')

    # Commands
    commands = tk.Frame(root)
    add_folder_button = ttk.Button(commands, image=folder_image,  text='Add Folder...', compound=tk.LEFT)
    add_folder_button.grid(row=1, column=1, sticky=tk.E)
    add_file_button = ttk.Button(commands, image=file_image,  text='Add File...', compound=tk.LEFT)
    add_file_button.grid(row=1, column=2, sticky=tk.E)
    commands.pack(side=tk.TOP, fill=tk.X, padx=(5,5))

    # files and folders
    frame = tk.Frame(root)
    frame.pack(side=tk.TOP, fill=tk.BOTH, expand=True)

    listbox = ttk.Treeview(frame, columns=('type', 'path'), selectmode='browse')
    listbox.heading('#0', text='Name')
    listbox.heading('type', text='Type')
    listbox.column('type', width=100)
    listbox.heading('path', text='Path')

    xscrollbar = tk.Scrollbar(frame, orient=tk.HORIZONTAL)
    xscrollbar.pack(side = tk.BOTTOM, fill=tk.X)
    listbox.config(xscrollcommand = xscrollbar.set)
    xscrollbar.config(command = listbox.xview)

    yscrollbar = tk.Scrollbar(frame, orient=tk.VERTICAL)
    yscrollbar.pack(side = tk.RIGHT, fill=tk.Y)
    listbox.config(yscrollcommand = yscrollbar.set)
    yscrollbar.config(command = listbox.yview)

    listbox.pack(side = tk.LEFT, fill = tk.BOTH, expand=True)
    listbox.bind('<<TreeviewSelect>>', lambda e: print(f'selection changed: {listbox.focus()}'))


    for i in range(0, 10):
        listbox.insert('', tk.END, text=f'item #{i}', image=file_image, values=('Datei' , '/some/path'))

    # selected item
    frame2 = tk.Frame(root)
    frame2.columnconfigure(1, weight=0)
    frame2.columnconfigure(2, weight=1)
    frame2.columnconfigure(3, weight=0)

    name_label = tk.Label(frame2, text='Name:')
    name_label.grid(row=1, column=1, sticky=tk.W, padx=(5,5))
    name_entry = tk.Entry(frame2, text='')
    name_entry.grid(row=1, column=2, columnspan=2, sticky=tk.EW, padx=(5,5))

    path_label = tk.Label(frame2, text='Path:')
    path_label.grid(row=2, column=1, sticky=tk.W, padx=(5,5))
    path_entry = tk.Entry(frame2, text='')
    path_entry.grid(row=2, column=2, sticky=tk.EW, padx=(5,5))
    path_select = ttk.Button(frame2, text='Select...', image=exchange_image, compound=tk.LEFT)
    path_select.grid(row=2, column=3, sticky=tk.W, padx=(5,5))

    frame2.pack(side=tk.TOP, fill=tk.BOTH)

    selection_commands = tk.Frame(root)
    selection_commands.pack(side=tk.TOP, fill=tk.X, padx=(5,5))

    update_button = ttk.Button(selection_commands, text='Update', image=update_image, compound=tk.LEFT)
    update_button.pack(side=tk.LEFT)

    update_button = ttk.Button(selection_commands, text='Remove', image=remove_image, compound=tk.LEFT)
    update_button.pack(side=tk.LEFT)

    # backup
    sep = ttk.Separator(root, orient=tk.HORIZONTAL)
    sep.pack(side=tk.TOP, fill=tk.X, pady=10)

    backup_commands = tk.Frame(root)
    backup_commands.pack(side=tk.TOP, fill=tk.X, padx=(5,5))

    backup_button = ttk.Button(backup_commands, text='Start Backup...', image=start_image, compound=tk.LEFT)
    backup_button.pack(side=tk.LEFT)

    progressbar = ttk.Progressbar(root, orient=tk.HORIZONTAL)
    progressbar.pack(side=tk.TOP, fill=tk.X, padx=(5,5), pady=(5,5))

    root.mainloop()



if __name__ == '__main__':
    main()