import * as vscode from 'vscode';

export function activate(context: vscode.ExtensionContext) {
    console.log('Mouse middle button shortcut extension is now active');

    // Middle mouse button (Command+Click equivalent)
    const middleClickDisposable = vscode.commands.registerCommand('vs.middleClick', async () => {
        // Show a "Hello World" message
        vscode.window.showInformationMessage('Hello World! Middle mouse button clicked.');
        
        // Perform the Command+Click equivalent action
        await vscode.commands.executeCommand('editor.action.revealDefinition');
    });

	const helloWorldDisposable = vscode.commands.registerCommand('vs.helloWorld', () => {
		vscode.window.showInformationMessage('Hello World from vs extension!');
	});

    context.subscriptions.push(middleClickDisposable, helloWorldDisposable);
}

export function deactivate() {}