const vscode = require('vscode');
const cp = require('child_process')

function trimPath(path) {
	let trimmedPath = path.replace('problem.yaml', '').replace('solution.rs', '').replace(/data\/secret.*/, '');
	if (path === trimmedPath) throw 'File selection not recognized';
	return trimmedPath;
}

function activate(context) {
	let openStatement = vscode.commands.registerCommand('cca.view.statement', () => {
		renderWebview(trimPath(vscode.window.activeTextEditor.document.uri.path) + 'statement.pdf');
	});
	context.subscriptions.push(openStatement);
	let openAnalysis = vscode.commands.registerCommand('cca.view.analysis', () => {
		renderWebview(trimPath(vscode.window.activeTextEditor.document.uri.path) + 'analysis.pdf');
	});
	context.subscriptions.push(openAnalysis);
	let createSolution = vscode.commands.registerCommand('cca.create', () => {
		let solutionPath = trimPath(vscode.window.activeTextEditor.document.uri.path) + 'solution.rs';
		let solutionName = solutionPath.replace('/solution.rs', '').split('/').pop();
		let configPath = solutionPath.split('coding-competitions-archive')[0] + 'coding-competitions-archive/Cargo.toml'
		cp.exec(`
		printf 'lib::run!();\n\nstruct TestCase {}\n\nfn read() -> TestCase { todo!() }\n\nfn solve(TestCase {}: TestCase) -> String { todo!() }\n' > ${solutionPath}
		printf "\n[[bin]]\nname = \\"${solutionName}\\"\npath = \\"${solutionPath.split('coding-competitions-archive/')[1]}\\"\n" >> ${configPath}
		`);
	});
	context.subscriptions.push(createSolution);
	let editSmallTestSet = vscode.commands.registerCommand('cca.edit_small_test_set', () => {
		cp.exec(`codium -r ${trimPath(vscode.window.activeTextEditor.document.uri.path) + 'data/secret/subtask1/1.in'}`);
	});
	context.subscriptions.push(editSmallTestSet);
	let runSmallTestSet = vscode.commands.registerCommand('cca.run_small_test_set', () => {
		runTestSet('subtask1');
	});
	context.subscriptions.push(runSmallTestSet);
	let runLargeTestSet = vscode.commands.registerCommand('cca.run_large_test_set', () => {
		runTestSet('subtask2');
	});
	context.subscriptions.push(runLargeTestSet);
}

function renderWebview(pdfPath) {
	let outputPrefix = `/tmp/${new Date().getTime()}/`;
	const panel = vscode.window.createWebviewPanel(
		'ccaPreview',
		`${pdfPath}`
	);
	panel.onDidChangeViewState(_ => {
		if (panel.visible) {
			setTimeout(() => panel.reveal(), 128);
		}
	});
	cp.exec(`
			mkdir ${outputPrefix}
			pdftoppm ${pdfPath} ${outputPrefix}raw -png
			convert ${outputPrefix}raw-*.png -colorspace Gray +level-colors "#bbbbbb","#22272e" ${outputPrefix}dark.png
			convert -append ${outputPrefix}dark-*png ${outputPrefix}dark.png
			base64 ${outputPrefix}dark.png
		`, { maxBuffer: 1024 * 1024 * 20 }, (_, stdout, __) => {
		panel.webview.html = `
		<!DOCTYPE html>
		<html lang="en">
		<head>
			<meta charset="UTF-8">
			<meta name="viewport" content="width=device-width, initial-scale=1.0">
		</head>
		<body>
		<div style="margin: auto; width: 900px;">
			<img src="data:image/png;base64,${stdout}"></img>
		</div>
		</body>
		</html>`;
	});
}

function runTestSet(testSet) {
	let solutionPath = trimPath(vscode.window.activeTextEditor.document.uri.path) + 'solution.rs';
	let solutionName = solutionPath.replace('/solution.rs', '').split('/').pop();
	let rootPath = solutionPath.split('coding-competitions-archive')[0] + 'coding-competitions-archive';
	let smallTestSetPath = solutionPath.replace(/[a-z\.]+$/, `data/secret/${testSet}/1.in`);
	let outputPath = solutionPath.replace(/[a-z\.]+$/, `data/secret/${testSet}/1.ans`);
	cp.exec(`touch ${outputPath} && codium -r ${outputPath}`, () => {
		cp.exec(`
			cd ${rootPath} &&
			cargo build --release --bin ${solutionName} 2> ${outputPath} &&
			${rootPath}/target/release/${solutionName} < ${smallTestSetPath} > ${outputPath}
		`);
	});
}

function deactivate() { }

module.exports = {
	activate,
	deactivate
}
