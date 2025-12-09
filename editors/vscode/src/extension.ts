import * as path from 'path';
import { workspace, ExtensionContext } from 'vscode';
import {
    LanguageClient,
    LanguageClientOptions,
    ServerOptions,
} from 'vscode-languageclient/node';

let client: LanguageClient;

export function activate(context: ExtensionContext) {
    const serverExecutable = workspace.getConfiguration('ssl').get<string>('executablePath') || 'ssl';
    
    const serverOptions: ServerOptions = {
        command: serverExecutable,
        args: ['lsp'],
    };

    const clientOptions: LanguageClientOptions = {
        documentSelector: [{ scheme: 'file', language: 'ssl' }],
        synchronize: {
            fileEvents: workspace.createFileSystemWatcher('**/*.ssl'),
        },
    };

    client = new LanguageClient(
        'sslLanguageServer',
        'SSL Language Server',
        serverOptions,
        clientOptions
    );

    client.start();
}

export function deactivate(): Thenable<void> | undefined {
    if (!client) {
        return undefined;
    }
    return client.stop();
}
