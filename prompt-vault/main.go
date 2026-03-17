package main

import (
	"fmt"
	"os"

	"github.com/spf13/cobra"
)

var (
	version = "0.1.0"
	rootCmd = &cobra.Command{
		Use:     "prompt-vault",
		Version: version,
		Short:   "🗄️ 你的个人提示词保险库 - 结构化管理所有AI提示词",
		Long: `Prompt Vault is a personal prompt management tool for AI users.
It helps you organize, search, and version control your AI prompts locally.`,
	}
)

func init() {
	rootCmd.AddCommand(initCmd())
	rootCmd.AddCommand(addCmd())
	rootCmd.AddCommand(listCmd())
	rootCmd.AddCommand(searchCmd())
	rootCmd.AddCommand(tuiCmd())
	rootCmd.AddCommand(exportCmd())
	rootCmd.AddCommand(importCmd())
}

func initCmd() *cobra.Command {
	var path string
	cmd := &cobra.Command{
		Use:   "init",
		Short: "Initialize a new prompt vault",
		Run: func(cmd *cobra.Command, args []string) {
			if path == "" {
				path = os.Getenv("HOME") + "/.prompt-vault"
			}
			
			if _, err := os.Stat(path); !os.IsNotExist(err) {
				fmt.Printf("Vault already exists at %s\n", path)
				return
			}
			
			os.MkdirAll(path+"/prompts", 0755)
			fmt.Printf("✅ Initialized prompt vault at %s\n", path)
		},
	}
	cmd.Flags().StringVar(&path, "path", "", "Path to initialize the vault (default: ~/.prompt-vault)")
	return cmd
}

func addCmd() *cobra.Command {
	cmd := &cobra.Command{
		Use:   "add",
		Short: "Add a new prompt",
		Run: func(cmd *cobra.Command, args []string) {
			fmt.Println("📝 Adding new prompt... (interactive mode coming soon)")
		},
	}
	return cmd
}

func listCmd() *cobra.Command {
	cmd := &cobra.Command{
		Use:   "list",
		Short: "List all prompts",
		Run: func(cmd *cobra.Command, args []string) {
			fmt.Println("📋 Listing prompts...")
		},
	}
	return cmd
}

func searchCmd() *cobra.Command {
	cmd := &cobra.Command{
		Use:   "search [query]",
		Short: "Search prompts by keyword",
		Args:  cobra.MinimumNArgs(1),
		Run: func(cmd *cobra.Command, args []string) {
			query := args[0]
			fmt.Printf("🔍 Searching for: %s\n", query)
		},
	}
	return cmd
}

func tuiCmd() *cobra.Command {
	cmd := &cobra.Command{
		Use:   "tui",
		Short: "Start interactive terminal UI",
		Run: func(cmd *cobra.Command, args []string) {
			fmt.Println("🎨 Starting interactive terminal UI...")
		},
	}
	return cmd
}

func exportCmd() *cobra.Command {
	var output string
	cmd := &cobra.Command{
		Use:   "export [output-file]",
		Short: "Export all prompts to JSON file",
		Args:  cobra.MaximumNArgs(1),
		Run: func(cmd *cobra.Command, args []string) {
			if len(args) > 0 {
				output = args[0]
			}
			fmt.Printf("📤 Exporting prompts to %s\n", output)
		},
	}
	return cmd
}

func importCmd() *cobra.Command {
	var input string
	cmd := &cobra.Command{
		Use:   "import [input-file]",
		Short: "Import prompts from JSON file",
		Args:  cobra.MinimumNArgs(1),
		Run: func(cmd *cobra.Command, args []string) {
			input = args[0]
			fmt.Printf("📥 Importing prompts from %s\n", input)
		},
	}
	return cmd
}

func main() {
	if err := rootCmd.Execute(); err != nil {
		fmt.Fprintln(os.Stderr, err)
		os.Exit(1)
	}
}
