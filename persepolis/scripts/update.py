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

try:
    from PySide6 import QtCore
    from PySide6.QtCore import QCoreApplication, QLocale, QPoint, QSettings, QSize, Qt, QTranslator
    from PySide6.QtGui import QCloseEvent, QIcon, QKeyEvent
    from PySide6.QtWidgets import QHBoxLayout, QLabel, QPushButton, QVBoxLayout, QWidget
except ImportError:
    from PyQt5 import QtCore
    from PyQt5.QtCore import QCoreApplication, QLocale, QPoint, QSettings, QSize, Qt, QTranslator
    from PyQt5.QtGui import QCloseEvent, QIcon, QKeyEvent
    from PyQt5.QtWidgets import QHBoxLayout, QLabel, QPushButton, QVBoxLayout, QWidget


import ast
import platform

import requests

import ghermez
from persepolis.constants import APP_NAME, LONG_NAME, OS, REPO_LINK, VERSION

# finding os_type
os_type = platform.system()


class checkupdate(QWidget):  # noqa: N801
    def __init__(self, persepolis_setting: QSettings) -> None:
        super().__init__()

        # defining UI
        self.persepolis_setting = persepolis_setting
        self.setWindowIcon(QIcon.fromTheme(APP_NAME, QIcon(':/ghermez.png')))

        # add support for other languages
        locale = str(self.persepolis_setting.value('settings/locale'))
        QLocale.setDefault(QLocale(locale))
        self.translator = QTranslator()
        if self.translator.load(':/translations/locales/ui_' + locale, 'ts'):
            QCoreApplication.installTranslator(self.translator)

        self.setWindowTitle(QCoreApplication.translate('update_src_ui_tr', 'Checking for newer version'))

        # installed version
        self.client_version = '3.20'

        # first line text
        self.update_label = QLabel(QCoreApplication.translate(
            'update_src_ui_tr', f'The newest is the best, we recommend to update {APP_NAME.capitalize()}.'))
        self.update_label.setTextFormat(QtCore.Qt.RichText)
        self.update_label.setAlignment(QtCore.Qt.AlignCenter)

        # second line text
        self.version_label = QLabel(QCoreApplication.translate(
            'update_src_ui_tr', f'This is {LONG_NAME} version {VERSION}'))
        self.version_label.setAlignment(QtCore.Qt.AlignCenter)

        # release link
        self.link_label = QLabel(
            f'<a href={REPO_LINK}/releases>{REPO_LINK}/releases</a>')
        self.link_label.setAlignment(QtCore.Qt.AlignCenter)
        self.link_label.setOpenExternalLinks(True)

        # version status
        self.status_label = QLabel()
        self.status_label.setTextFormat(QtCore.Qt.RichText)
        self.status_label.setAlignment(QtCore.Qt.AlignCenter)

        # update button
        self.check_button = QPushButton(QCoreApplication.translate('update_src_ui_tr', 'Check for new update'))
        self.check_button.clicked.connect(self.updateCheck)

        # verticalLayout
        vbox = QVBoxLayout()
        vbox.addWidget(self.update_label)
        vbox.addWidget(self.version_label)
        vbox.addWidget(self.link_label)
        vbox.addWidget(self.check_button)
        vbox.addWidget(self.status_label)

        # horizontalLayout
        hbox = QHBoxLayout()
        hbox.addLayout(vbox)

        # window layout
        self.setLayout(hbox)

        # window size and position
        size = self.persepolis_setting.value(
            'checkupdate/size', QSize(360, 250))
        position = self.persepolis_setting.value(
            'checkupdate/position', QPoint(300, 300))

        self.resize(size)
        self.move(position)

    # checking methode
    def updateCheck(self, _button: QPushButton) -> None:
        self.check_button.setText(QCoreApplication.translate('update_src_ui_tr', 'Checking...'))

        try:
            # get information dictionary from github
            updatesource = requests.get('https://persepolisdm.github.io/version')  # noqa: S113

            updatesource_text = updatesource.text
            updatesource_dict = ast.literal_eval(updatesource_text)

            # get latest stable version
            server_version = updatesource_dict['version']

            # Comparison
            if float(server_version) > float(self.client_version):
                self.status_label.setText(QCoreApplication.translate(
                    'update_src_ui_tr', f'A newer {APP_NAME.capitalize()} release is available'))

                if os_type == OS.WINDOWS:
                    self.winUpdatedl()  # this function download latest release

                    # find system architect
                    if platform.architecture()[0] == '64bit':

                        ghermez.xdgOpen(updatesource_dict['win64dlurl'])

                    elif platform.architecture()[0] == '32bit':

                        ghermez.xdgOpen(updatesource_dict['win32dlurl'])

                elif os_type == OS.OSX:
                    ghermez.xdgOpen(updatesource_dict['macdlurl'])  # it will download latest release for mac

            elif float(server_version) == float(self.client_version):
                self.status_label.setText(QCoreApplication.translate(
                    'update_src_ui_tr', 'Latest version is installed :)'))

            elif float(server_version) < float(self.client_version):
                self.status_label.setText(QCoreApplication.translate('update_src_ui_tr', 'You are using beta version'))

        except Exception:
            self.status_label.setText(QCoreApplication.translate(
                'update_src_ui_tr', 'An error occurred while checking for updates.'))

        self.check_button.setText(QCoreApplication.translate('update_src_ui_tr', 'Check for new update'))

    # close window with ESC key
    def keyPressEvent(self, event: QKeyEvent) -> None:
        if event.key() == Qt.Key_Escape:
            self.close()


    def closeEvent(self, event: QCloseEvent) -> None:
        # saving window size and position
        self.persepolis_setting.setValue(
            'checkupdate/size', self.size())
        self.persepolis_setting.setValue(
            'checkupdate/position', self.pos())
        self.persepolis_setting.sync()
        event.accept()
