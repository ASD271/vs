try to use rust device state to listen mouse click, but has some problem now.

mouse_listener/native/src/listen_mouse.rs is the file how to listen mouse

src/extension.ts is the file of vscode extension

```ts
mouseListener = listenMouse((error, result) => {
			vscode.window.showInformationMessage(result);
			if (result === 'middle') {
				vscode.commands.executeCommand('vs.goToDefinition');
			}
        });
```
now showInformationMessage work well. but revealDefinition( call by goToDefinition ) seem has been blocked.
