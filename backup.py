#!/usr/bin/env python3

"""Simple backup application."""

import os
import time
import argparse
from pathlib import Path
import tkinter as tk
from tkinter import ttk
from tkinter import filedialog
from PIL import Image, ImageTk
import yaml
import shutil
import hashlib

import tk_async_execute as tae

DEFAULT_TITLE = 'Backup'
DEFAULT_WIDTH = 640
DEFAULT_HEIGHT = 480

def load_image(filename):
    """Loads an image file."""
    img = Image.open(os.path.join('icons', filename))
    img = img.resize( (16,16))
    img2 = Image.new('RGBA', (24, 16))
    img2.paste(img)
    return ImageTk.PhotoImage(img2)

# pylint: disable-next=too-many-instance-attributes
class MainWindow:
    """MainWindow class."""

    # pylint: disable-next=too-many-locals,too-many-statements
    def __init__(self, project: str):
        root = tk.Tk()
        self.root = root
        root.title(DEFAULT_TITLE)
        root.iconbitmap(os.path.join('icons','logo.ico'))
        root.geometry(f'{DEFAULT_WIDTH}x{DEFAULT_HEIGHT}')
        root.minsize(DEFAULT_WIDTH, DEFAULT_HEIGHT)
        root.protocol("WM_DELETE_WINDOW", self.on_closing)

        self.project = project

        self.file_image = load_image('file.png')
        self.folder_image = load_image('folder.png')
        self.remove_image = load_image('remove.png')
        self.start_image = load_image('start.png')
        self.exchange_image = load_image('exchange.png')
        self.update_image = load_image('update.png')

        # Commands
        commands = tk.Frame(root)
        add_folder_button = ttk.Button(commands, image=self.folder_image,
            text='Add Folder...', compound=tk.LEFT, command=self.add_folder)
        add_folder_button.grid(row=1, column=1, sticky=tk.E)
        add_file_button = ttk.Button(commands, image=self.file_image,
            text='Add File...', compound=tk.LEFT, command=self.add_file)
        add_file_button.grid(row=1, column=2, sticky=tk.E)
        commands.pack(side=tk.TOP, fill=tk.X, padx=(5,5))

        # files and folders
        frame = tk.Frame(root)
        frame.pack(side=tk.TOP, fill=tk.BOTH, expand=True)

        listbox = ttk.Treeview(frame, columns=('type', 'path'), selectmode='browse')
        self.listbox = listbox
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
        listbox.bind('<<TreeviewSelect>>', self.on_selection_changed)

        # selected item
        frame2 = tk.Frame(root)
        frame2.columnconfigure(1, weight=0)
        frame2.columnconfigure(2, weight=1)
        frame2.columnconfigure(3, weight=0)

        self.name_var = tk.StringVar(frame2, value='')
        name_label = tk.Label(frame2, text='Name:')
        name_label.grid(row=1, column=1, sticky=tk.W, padx=(5,5))
        name_entry = tk.Entry(frame2, textvariable=self.name_var)
        name_entry.grid(row=1, column=2, columnspan=2, sticky=tk.EW, padx=(5,5))

        self.path_var = tk.StringVar(frame2, value='')
        path_label = tk.Label(frame2, text='Path:')
        path_label.grid(row=2, column=1, sticky=tk.W, padx=(5,5))
        path_entry = tk.Entry(frame2, textvariable=self.path_var)
        path_entry.grid(row=2, column=2, sticky=tk.EW, padx=(5,5))
        path_select = ttk.Button(frame2, text='Select...',
            image=self.exchange_image, compound=tk.LEFT, command=self.select_path)
        path_select.grid(row=2, column=3, sticky=tk.W, padx=(5,5))
        self.path_select = path_select

        frame2.pack(side=tk.TOP, fill=tk.BOTH)

        selection_commands = tk.Frame(root)
        selection_commands.pack(side=tk.TOP, fill=tk.X, padx=(5,5))

        update_button = ttk.Button(selection_commands, text='Update',
            image=self.update_image, compound=tk.LEFT, command=self.update_selected)
        update_button.pack(side=tk.LEFT)
        self.update_button = update_button

        remove_button = ttk.Button(selection_commands, text='Remove',
            image=self.remove_image, compound=tk.LEFT, command=self.remove_selected)
        remove_button.pack(side=tk.LEFT)
        self.remove_button = remove_button

        # backup
        sep = ttk.Separator(root, orient=tk.HORIZONTAL)
        sep.pack(side=tk.TOP, fill=tk.X, pady=10)

        backup_commands = tk.Frame(root)
        backup_commands.pack(side=tk.TOP, fill=tk.X, padx=(5,5))

        backup_button = ttk.Button(backup_commands, text='Start Backup...',
            image=self.start_image, compound=tk.LEFT, command=self.start_backup)
        backup_button.pack(side=tk.LEFT)

        self.progressbar = ttk.Progressbar(root, orient=tk.HORIZONTAL)
        self.progressbar.pack(side=tk.TOP, fill=tk.X, padx=(5,5), pady=(5,5))

        # load project
        items = self.load_project()
        print(f'{items}')
        for item in items:
            name = item['name']
            itemtype = item['type']
            path = item['path']
            image = self.get_image(itemtype)
            self.listbox.insert('', tk.END, text=name, image=image, values=(itemtype , path))
        self.on_selection_changed(None)

    def get_image(self, itemtype: str):
        """Returns an image by the provided items type."""
        if itemtype == 'Folder':
            return self.folder_image
        if itemtype == 'File':
            return self.file_image
        return self.file_image

    def load_project(self):
        """Loads the project file."""
        items = []
        try:
            with open(self.project, 'r', encoding='utf-8') as f:
                doc = yaml.load(f, Loader=yaml.SafeLoader)
                if doc['schema_version'] != 1:
                    raise AssertionError('unknown schema')
                for item in doc['items']:
                    items.append(item)
        # pylint: disable-next=broad-exception-caught
        except Exception as ex:
            print(f'warn: failed to load project \'{self.project}\': {ex}')
        return items

    def save_project(self):
        """Saves the proves file."""
        try:
            items = []
            for itemid in self.listbox.get_children():
                item = self.listbox.item(itemid)
                name = item['text']
                itemtype = item['values'][0]
                path = item['values'][1]
                items.append({'name': name, 'type': itemtype, 'path': path})
            data = {'schema_version': 1, 'items': items}
            with open(self.project, 'w', encoding='utf-8') as f:
                f.write(yaml.dump(data, sort_keys=False))
        # pylint: disable-next=broad-exception-caught
        except Exception as ex:
            print(f'failed to save \'{self.project}\': {ex}')

    def add_folder(self):
        """Adds a new folder."""
        folder = filedialog.askdirectory()
        if folder:
            name = os.path.basename(folder)
            self.listbox.insert('', tk.END, text=name,
                image=self.folder_image, values=('Folder' , folder))

    def add_file(self):
        """Adds a new file."""
        file = filedialog.askopenfilename()
        if file:
            name = os.path.basename(file)
            self.listbox.insert('', tk.END, text=name,
                image=self.file_image, values=('File' , file))

    def update_selected(self):
        """Updates the seletect items."""
        selected = self.listbox.focus()
        if selected:
            item = self.listbox.item(selected)
            itemtype = item['values'][0]
            self.listbox.item(selected, text=self.name_var.get(),
                values=[itemtype ,self.path_var.get()])

    def remove_selected(self):
        """Removes the selected item."""
        selected = self.listbox.focus()
        if selected:
            self.listbox.delete(selected)

    # pylint: disable-next=unused-argument
    def on_selection_changed(self, e):
        """Updates the displayed values to match the newly selected item."""
        selected = self.listbox.focus()
        if selected:
            item = self.listbox.item(selected)
            name = item['text']
            path = item['values'][1]
            self.name_var.set(name)
            self.path_var.set(path)
            self.update_button['state'] = 'normal'
            self.remove_button['state'] = 'normal'
            self.path_select['state'] = 'normal'
        else:
            self.name_var.set('')
            self.path_var.set('')
            self.update_button['state'] = 'disabled'
            self.remove_button['state'] = 'disabled'
            self.path_select['state'] ='disabled'

    def select_path(self):
        """Updates the path of the selected item."""
        selected = self.listbox.focus()
        if selected:
            item = self.listbox.item(selected)
            itemtype = item['values'][0]
            path = item['values'][1]
            if itemtype == 'Folder':
                folder = filedialog.askdirectory(initialdir=path)
                if folder:
                    self.path_var.set(folder)
            elif itemtype == 'File':
                file = filedialog.askopenfilename(initialfile=path)
                if file:
                    self.path_var.set(file)
            else:
                # Do nothing for unknown type
                pass

    def start_backup(self):
        """Starts the backup."""
        folder = filedialog.askdirectory()
        if folder:
            print('backup started')
            items = []
            for itemid in self.listbox.get_children():
                item = self.listbox.item(itemid)
                name = item['text']
                itemtype = item['values'][0]
                path = item['values'][1]
                items.append({'name': name, 'type': itemtype, 'path': path})
            self.progressbar.start()
            tae.async_execute(self.do_backup(items, folder), callback=self.finish_backup, wait=False, visible=False)

    async def do_backup(self, items, folder):
        """Performs the actual backup"""
        # get file list
        if os.listdir(folder):
            print(f'error: folder is not empty')
            return

        files = []
        for item in items:
            itemtype = item['type']
            if itemtype == 'Folder':
                path = item['path']
                for dir, dname, fnames in os.walk(item['path']):
                    targetdir = dir[len(os.path.commonpath([path, dir])):]
                    if targetdir.startswith('/'):
                        targetdir = targetdir[1:]
                    targetdir = os.path.join(item['name'], targetdir)
                    for fname in fnames:
                        source = os.path.join(dir, fname)                        
                        target = os.path.join(targetdir, fname)
                        files.append({'source': source, 'target': target})
            elif itemtype == 'File':
                files.append({'source': item['path'], 'target': item['name']})

        with open(os.path.join(folder, 'checksums.txt'), 'w', newline='\n', encoding='utf-8') as checksum_file:
            for file in files:
                source = file['source']
                rel_target = file['target']
                target = os.path.join(folder, rel_target)       
                os.makedirs(os.path.dirname(target), exist_ok=True)
                shutil.copy2(source, target)
                with open(target, 'rb') as f:
                    digest = hashlib.file_digest(f, "sha256")
                    checksum_file.write(f'SHA256 ({rel_target}) = {digest.hexdigest()}\n')        

    def on_closing(self):
        """Saves the project when the application is closed."""
        self.save_project()
        self.root.destroy()

    def finish_backup(self):
        """Finished the backup."""
        print('backup finished')
        self.progressbar.stop()

    def run(self):
        """Runs the application."""
        tae.start()
        self.root.mainloop()
        tae.stop()

def main():
    """Setup and run the aplication."""
    default_project = os.path.join(Path.home(), 'backup-py.yaml')
    parser = argparse.ArgumentParser()
    parser.add_argument('-p', '--project', type=str, help='project file', default=default_project)
    args = parser.parse_args()
    app = MainWindow(args.project)
    app.run()


if __name__ == '__main__':
    main()
