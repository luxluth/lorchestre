import QtQuick
import QtQuick.Window
import Lorchestre 1.0

Window {
    visible: true
    // Instantiate the rust struct
    Text {
        anchors.centerIn: parent
        // Call a method
        text: "hello"
    }
}
