import QtQuick 2.12
import QtQuick.Controls 2.12
import QtQuick.Window 2.12

// This must match the qml_uri and qml_version
// specified with the #[cxx_qt::qobject] macro in Rust.
import caesar 1.0

Window {
    title: qsTr("Caesar")
    visible: true
    height: 480
    width: 640
    color: "#e4af79"

    Rot {
        id: rot
        plain: ""
        secret: ""
    }

    Column {
        anchors.horizontalCenter: parent.horizontalCenter
        anchors.verticalCenter: parent.verticalCenter
        /* space between widget */
        spacing: 10

        Label {
            text: "Keep your secret safe 🔒"
            font.bold: true
        }

        TextArea {
            id: plain
            placeholderText: qsTr("me@caesar.tld")
            text: rot.plain
            onTextChanged: rot.secret = rot.encrypt(text)

            background: Rectangle {
                implicitWidth: 400
                implicitHeight: 50
                radius: 3
                color: plain.enabled ? "#e2e8f0" : "#353637"
                border.color: plain.enabled ? "#21be2b" : "#e2e8f0"
            }
        }

        TextArea {
            id: sercet
            placeholderText: qsTr("zr@pnrfne.gyq")
            text: rot.secret
            onTextChanged: rot.plain = rot.decrypt(text)

            background: Rectangle {
                implicitWidth: 400
                implicitHeight: 50
                radius: 3
                color: sercet.enabled ? "#e2e8f0" : "#353637"
                border.color: sercet.enabled ? "#21be2b" : "#e2e8f0"
            }
        }
    }
}
