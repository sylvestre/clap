# Chaînes de localisation clap principales - Français

# Aide et version
print-help = Afficher l'aide
print-version = Afficher la version
clap-print-help = Affiche ce message ou l'aide de la/des sous-commande(s) donnée(s)

# Formatage de l'usage
usage-header = Utilisation
usage-options = OPTIONS
usage-default-subcommand = COMMANDE

# Messages d'erreur
clap-error-arg-undefined = L'argument `{$id}` n'est pas défini
clap-error-group-undefined = Le groupe `{$id}` n'est pas défini
clap-error-command-undefined = La commande `{$name}` n'est pas définie

# Messages d'aide pour les fonctionnalités intégrées
help-short-help = Afficher l'aide (voir plus avec '--help')
help-long-help = Afficher l'aide (voir un résumé avec '-h')
help-subcommand-help = Afficher l'aide pour la/les sous-commande(s)

# Descriptions d'application modèles
app-about = Une application CLI d'exemple avec support de localisation complet
app-long-about = Cette application démontre comment intégrer la localisation dans un outil en ligne de commande basé sur clap en utilisant l'approche anglaise en ligne améliorée.
app-after-help = Pour plus d'informations, visitez : https://example.com/docs

# Messages d'aide pour les arguments
arg-config-help = Définit un fichier de configuration personnalisé
arg-config-long-help = Spécifie le chemin vers un fichier de configuration. Si non fourni, la configuration par défaut sera utilisée.
arg-verbose-help = Augmente la verbosité (utilisez plusieurs fois pour une sortie plus détaillée)
arg-output-help = Répertoire de sortie
arg-format-help = Format de sortie
arg-input-help = Fichier d'entrée à traiter
arg-threads-help = Nombre de threads de traitement
arg-strict-help = Active le mode de validation strict

# Descriptions des commandes
cmd-process-about = Traite le fichier d'entrée
cmd-validate-about = Valide le fichier d'entrée

# Formatage de l'aide
help-usage-line = my-cli-app [OPTIONS] <ENTRÉE> [COMMANDE]
help-description = Une application CLI d'exemple démontrant les fonctionnalités de localisation.
help-options-header = OPTIONS :
help-commands-header = COMMANDES :
help-commands = Commande
help-examples-header = EXEMPLES :

# Messages informatifs
info-verbose-level = Niveau de verbosité défini à { $level }
info-using-config = Utilisation du fichier de configuration : { $file }
info-output-format = Format de sortie : { $format }
info-default-action = Exécution de l'action par défaut sur le fichier : { $file }
info-processing-file = Traitement du fichier '{ $file }' avec { $threads } threads
info-progress-update = Étape de traitement { $step } sur 5...
info-processing-complete = Fichier traité avec succès : { $file }
info-validating-file = Validation du fichier '{ $file }' en { $mode }
info-validation-passed = Le fichier '{ $file }' a passé la validation

# Modes de validation
validation-mode-strict = mode strict
validation-mode-normal = mode normal

# Messages d'erreur
error-file-not-found = Fichier non trouvé : { $path }
error-validation-strict-failed = La validation stricte a échoué pour le fichier '{ $file }' : format non supporté
error-unrecognized-subcommand = Sous-commande non reconnue : { $subcommand }

# Système d'erreur principal
error-unknown-cause = cause inconnue
error-label = erreur
error-tip = conseil

# Erreurs de conflit d'arguments
error-argument-cannot-be-used-multiple-times = l'argument '{ $argument }' ne peut pas être utilisé plusieurs fois
error-argument-cannot-be-used-with = l'argument '{ $argument }' ne peut pas être utilisé avec
error-subcommand-cannot-be-used-with = la sous-commande '{ $subcommand }' ne peut pas être utilisée avec
error-one-or-more-other-arguments = un ou plusieurs des autres arguments spécifiés

# Erreurs de valeur et d'assignation
error-equal-sign-needed = un signe égal est nécessaire lors de l'assignation de valeurs à '{ $argument }'
error-value-required-but-none-supplied = une valeur est requise pour '{ $argument }' mais aucune n'a été fournie
error-invalid-value-for-argument = valeur invalide '{ $value }' pour '{ $argument }'
error-possible-values = valeurs possibles

# Erreurs de sous-commande
error-requires-subcommand = '{ $command }' nécessite une sous-commande mais aucune n'a été fournie
error-subcommands = sous-commandes

# Arguments manquants
error-missing-required-arguments = les arguments requis suivants n'ont pas été fournis

# Erreurs de nombre de valeurs
error-unexpected-value-no-more-expected = valeur inattendue '{ $value }' pour '{ $argument }' trouvée ; aucune autre n'était attendue
error-values-required-only-provided = { $min_values } valeurs requises par '{ $argument }' ; seulement { $actual_values } { $were_provided }
error-wrong-number-of-values = { $expected_values } valeurs requises pour '{ $argument }' mais { $actual_values } { $were_provided }
error-were-provided = ont été fournies
error-was-provided = a été fournie

# Erreurs d'argument inconnu
error-unexpected-argument = argument inattendu '{ $argument }' trouvé

# Messages d'aide
error-for-more-information-try = Pour plus d'informations, essayez '{ $help }'.

# Types de contexte pour les suggestions
error-context-subcommand = sous-commande
error-context-argument = argument
error-context-value = valeur
error-context-subcommands = sous-commandes
error-context-arguments = arguments
error-context-values = valeurs

# Messages de suggestion
error-similar-exists-singular = un { $context } similaire existe : '{ $suggestion }'
error-similar-exists-plural = des { $context } similaires existent : '{ $suggestion }'
