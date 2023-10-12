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

import ast


# This function is writting a list in file_path in dictionary format
def writeList(file_path, dir_list):
    dictionary = {'list': dir_list}
    f = open(file_path, 'w')
    f.writelines(str(dictionary))
    f.close()

# This function is reading file_path and return content of file in list format


def readList(file_path, mode='dictionary'):
    f = open(file_path)
    f_string = f.readline()
    f.close()
    dictionary = ast.literal_eval(f_string.strip())
    dir_list = dictionary['list']

    if mode == 'string':
        dir_list[9] = str(dir_list[9])

    return dir_list

# this function is reading a file that contains dictionary , and extracts
# dictionary from it.


def readDict(file_path):
    f = open(file_path)
    f_lines = f.readlines()
    f.close()
    dict_str = str(f_lines[0].strip())
    return ast.literal_eval(dict_str)
