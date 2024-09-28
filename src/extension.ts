import * as vscode from 'vscode';

export function activate(context: vscode.ExtensionContext) {
	console.log('Mouse middle button shortcut extension is now active');

	const helloWorldDisposable = vscode.commands.registerCommand('vs.helloWorld', () => {
		vscode.window.showInformationMessage('Hello World from vs extension test2!');
	});

	const goToDefinitionDisposable = vscode.commands.registerCommand('vs.goToDefinition', async () => {
		vscode.window.showInformationMessage('Middle mouse button clicked.');
		const editor = vscode.window.activeTextEditor;
		if (!editor) {
			return;
		}

		const position = editor.selection.active;

		// 尝试执行 Go to Definition
		try {
			await vscode.commands.executeCommand('editor.action.revealDefinition');
		} catch (error) {
			console.error('无法执行 Go to Definition:', error);
			vscode.window.showErrorMessage('无法跳转到定义');
		}
	});
	context.subscriptions.push(helloWorldDisposable, goToDefinitionDisposable);
}

export function deactivate() { }