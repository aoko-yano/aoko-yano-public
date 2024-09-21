# 前提

- Windowsマシン（Windows 11）

# インストールするもの

- **pandoc**
	- version 3.4
- **MiKTeX**
	- version 24.1
		- インストールしたら一通りupdateをかけておく

# Obsidian側の準備

- ObsidianのCommunity Pluginsの**Enhancing Export**を入れておく

# 手順

- Markdownを左側のビューから右クリックで選び、**Export To...** を選択してMarkdownをtexファイルに変換
	- Typeは**Latex**
	- Latex Templateは**None**
- texファイルの以下の部分を手動で編集
	- **\\documentclass\[\]{article}** となっている箇所を、**\\documentclass\[\]{ltjsarticle}** にする
		- **ここがミソ１**
- texファイルからpdfファイルを生成
	- **LuaLaTeX**でpdfファイルを生成する
	- あるいは**lualatexコマンド**にtexファイルを流し込む
		- **ここがミソ２**