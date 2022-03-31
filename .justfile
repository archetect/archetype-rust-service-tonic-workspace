PascalCase := "CustomerHistory"
snake_case := "customer_history"
train-case := "customer-history"
pattern := '.*\.rs|.*\.md|.*\.proto|Cargo.toml|.*\.json|.*\.krop|.*\.krpref|.*.krproj'

fix:
  fd -0 -H '{{ pattern }}' . | xargs -0 sd -f c -s "{{ PascalCase }}Service" "{{{{ ArtifactId }}"
  fd -0 -H '{{ pattern }}' . | xargs -0 sd -f c -s "{{ snake_case }}_service" "{{{{ artifact_id }}"
  fd -0 -H '{{ pattern }}' . | xargs -0 sd -f c -s "{{ snake_case }}.service" "{{{{ prefix_name }}.{{{{ suffix_name }}"
  fd -0 -H '{{ pattern }}' . | xargs -0 sd -f c -s "{{ snake_case }}/service" "{{{{ prefix_name }}/{{{{ suffix_name }}"
  fd -0 -H '{{ pattern }}' . | xargs -0 sd -f c -s "{{ train-case }}-service" "{{{{ artifact-id }}"
  fd -0 -H '{{ pattern }}' . | xargs -0 sd -f c -s "{{ PascalCase }}" "{{{{ PrefixName }}"
  fd -0 -H '{{ pattern }}' . | xargs -0 sd -f c -s "{{ snake_case }}" "{{{{ prefix_name }}"
  fd -0 -H '{{ pattern }}' . | xargs -0 sd -f c -s "{{ train-case }}" "{{{{ prefix-name }}"
  fd -0 -H '{{ pattern }}' . | xargs -0 sd -f c -s {{"{{"}}"{" {{"{{"}}"'{'}}"{{"{{"}}
