#!/usr/bin/env python3
#    This program is free software: you can redistribute it and/or modify
#    it under the terms of the GNU General Public License as published by
#    the Free Software Foundation, either version 3 of the License, or
#    (at your option) any later version.
#
#    This program is distributed in the hope that it will be useful,
#    but WITHOUT ANY WARRANTY; without even the implied warranty of
#    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
#    GNU General Public License for more details.
#
#    You should have received a copy of the GNU General Public License
#    along with this program.  If not, see <http://www.gnu.org/licenses/>.
#


import glob
import os
import platform
import shutil
import sys

os_type = platform.system()

if os_type == 'Linux':
    path_list = ['/usr/share/man/man1/ghermez.1.gz',
                 '/usr/share/pixmaps/ghermez.svg',
                 '/usr/share/pixmaps/ghermez-tray.svg',
                 '/usr/share/applications/com.github.iamrezamousavi.ghermez.desktop',
                 '/usr/share/metainfo/com.github.iamrezamousavi.ghermez.appdata.xml',
                 '/usr/bin/ghermez']

elif os_type in ('FreeBSD', 'OpenBSD'):
    path_list = ['/usr/local/share/man/man1/ghermez.1.gz',
                 '/usr/local/share/pixmaps/ghermez.svg',
                 '/usr/local/share/pixmaps/ghermez-tray.svg',
                 '/usr/local/share/applications/com.github.iamrezamousavi.ghermez.desktop',
                 '/usr/local/share/metainfo/com.github.iamrezamousavi.ghermez.appdata.xml',
                 '/usr/local/bin/ghermez']

else:
    print('This script is for Linux and BSD')
    sys.exit(1)


# finding ghermez directories in /usr/lib/python3.8/site-packages/
python_version_list = ['python3.8', 'python3.9', 'python3.10', 'python3.11']

for python_version in python_version_list:
    # for BSD
    if os_type == 'Linux':

        pattern = '/usr/lib/' + python_version + '/site-packages/ghermez*'

    elif os_type in ('FreeBSD', 'OpenBSD'):

        pattern = '/usr/local/lib/' + python_version + '/site-packages/ghermez*'

    for folder in glob.glob(pattern):
        path_list.append(folder)

# finding persepolis directories in /usr/lib/python3.8/site-packages/
for python_version in python_version_list:
    # for BSD
    if os_type == 'Linux':

        pattern = '/usr/lib/' + python_version + '/site-packages/persepolis*'

    elif os_type in ('FreeBSD', 'OpenBSD'):

        pattern = '/usr/local/lib/' + python_version + '/site-packages/persepolis*'

    for folder in glob.glob(pattern):
        path_list.append(folder)

print(path_list)

uid = os.getuid()
if uid != 0:
    print('run this script as root.')
    sys.exit(1)


for path in path_list:
    if os.path.exists(path):
        if os.path.isfile(path):  # if path is for file
            os.remove(path)  # removing file
        else:
            shutil.rmtree(path)  # removing folder
        print(str(path) + ' is removed!')

print('uninstallation is complete!')
