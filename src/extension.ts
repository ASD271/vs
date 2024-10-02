import * as vscode from 'vscode';
import { listenMouse } from 'mouse_listener';

let mouseListener: any = null;
// mouseListener = listenMouse((error, result) => {
// 	if(error){
// 		console.error('鼠标监听错误:', error);
// 		return;
// 	}
// 	vscode.window.showInformationMessage(result);
// });
export function activate(context: vscode.ExtensionContext) {
    const helloWorldDisposable = vscode.commands.registerCommand('vs.helloWorld', () => {
        vscode.window.showInformationMessage('Hello World from vs extension test10!');
    });

    const goToDefinitionDisposable = vscode.commands.registerCommand('vs.goToDefinition', async () => {
        console.log('准备跳转到定义');

        const editor = vscode.window.activeTextEditor;
        if (!editor) {
            console.log('没有活动的编辑器');
            return;
        }
        console.log('找到活动编辑器');

        try {
            console.log('尝试执行跳转到定义');
            await vscode.commands.executeCommand('editor.action.revealDefinition');
        } catch (error) {
            console.error('无法执行跳转到定义:', error);
            vscode.window.showErrorMessage('无法跳转到定义');
        }
    });


    // 添加鼠标监听
    try {
        mouseListener = listenMouse((error, result) => {
			vscode.window.showInformationMessage(result);
			if (result === 'middle') {
				vscode.commands.executeCommand('vs.goToDefinition');
			}
        });
    } catch (error) {
        console.error('启动鼠标监听失败:', error);
    }

    context.subscriptions.push(helloWorldDisposable, goToDefinitionDisposable);
}

export function deactivate() {
    if (mouseListener && typeof mouseListener.dispose === 'function') {
        mouseListener.dispose();
    }
}
