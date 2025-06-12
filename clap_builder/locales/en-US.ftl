# Core clap localization strings - English

# Help and version
print-help = Print help
print-version = Print version
clap-print-help = Print this message or the help of the given subcommand(s)

# Usage formatting
usage-header = Usage
usage-options = OPTIONS
usage-default-subcommand = COMMAND

# Error messages
clap-error-arg-undefined = Argument `{$id}` is undefined
clap-error-group-undefined = Group `{$id}` is undefined
clap-error-command-undefined = Command `{$name}` is undefined

# Help messages for built-in functionality
help-short-help = Print help (see more with '--help')
help-long-help = Print help (see a summary with '-h')
help-subcommand-help = Print help for the subcommand(s)

# Application description placeholders
app-about = A sample CLI application with full localization support
app-long-about = This application demonstrates how to integrate localization into a clap-based command line tool using the improved inline English approach.
app-after-help = For more information, visit: https://example.com/docs

# Argument help messages
arg-config-help = Sets a custom config file
arg-config-long-help = Specifies the path to a configuration file. If not provided, the default configuration will be used.
arg-verbose-help = Increase verbosity (use multiple times for more verbose output)
arg-output-help = Output directory
arg-format-help = Output format
arg-input-help = Input file to process
arg-threads-help = Number of processing threads
arg-strict-help = Enable strict validation mode

# Command descriptions
cmd-process-about = Process the input file
cmd-validate-about = Validate the input file

# Help formatting
help-usage-line = my-cli-app [OPTIONS] <INPUT> [COMMAND]
help-description = A sample CLI application demonstrating localization features.
help-options-header = OPTIONS:
help-commands-header = COMMANDS:
help-commands = Commande
help-examples-header = EXAMPLES:

# Informational messages
info-verbose-level = Verbosity level set to { $level }
info-using-config = Using configuration file: { $file }
info-output-format = Output format: { $format }
info-default-action = Performing default action on file: { $file }
info-processing-file = Processing file '{ $file }' with { $threads } threads
info-progress-update = Processing step { $step } of 5...
info-processing-complete = Successfully processed file: { $file }
info-validating-file = Validating file '{ $file }' in { $mode }
info-validation-passed = File '{ $file }' passed validation

# Validation modes
validation-mode-strict = strict mode
validation-mode-normal = normal mode

# Error messages
error-file-not-found = File not found: { $path }
error-validation-strict-failed = Strict validation failed for file '{ $file }': unsupported format
error-unrecognized-subcommand = Unrecognized subcommand: { $subcommand }

# Core error system
error-unknown-cause = unknown cause
error-label = error
error-tip = tip

# Argument conflict errors
error-argument-cannot-be-used-multiple-times = the argument '{ $argument }' cannot be used multiple times
error-argument-cannot-be-used-with = the argument '{ $argument }' cannot be used with
error-subcommand-cannot-be-used-with = the subcommand '{ $subcommand }' cannot be used with
error-one-or-more-other-arguments = one or more of the other specified arguments

# Value and assignment errors
error-equal-sign-needed = equal sign is needed when assigning values to '{ $argument }'
error-value-required-but-none-supplied = a value is required for '{ $argument }' but none was supplied
error-invalid-value-for-argument = invalid value '{ $value }' for '{ $argument }'
error-possible-values = possible values

# Subcommand errors
error-requires-subcommand = '{ $command }' requires a subcommand but one was not provided
error-subcommands = subcommands

# Missing arguments
error-missing-required-arguments = the following required arguments were not provided

# Value count errors
error-unexpected-value-no-more-expected = unexpected value '{ $value }' for '{ $argument }' found; no more were expected
error-values-required-only-provided = { $min_values } values required by '{ $argument }'; only { $actual_values } { $were_provided }
error-wrong-number-of-values = { $expected_values } values required for '{ $argument }' but { $actual_values } { $were_provided }
error-were-provided = were provided
error-was-provided = was provided

# Unknown argument errors
error-unexpected-argument = unexpected argument '{ $argument }' found

# Help messages
error-for-more-information-try = For more information, try '{ $help }'.

# Context types for suggestions
error-context-subcommand = subcommand
error-context-argument = argument
error-context-value = value
error-context-subcommands = subcommands
error-context-arguments = arguments
error-context-values = values

# Suggestion messages
error-similar-exists-singular = a similar { $context } exists: '{ $suggestion }'
error-similar-exists-plural = some similar { $context } exist: '{ $suggestion }'
