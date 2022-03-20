PascalCase := "Customer"
snake_case := "customer"
pattern := '.*\.rs|.*\.md|.*\.proto|Cargo.toml'

fix:
  fd -0 '{{ pattern }}' . | xargs -0 sd -f c -s "{{ PascalCase }}Service" "{{{{ ArtifactId }}"
  fd -0 '{{ pattern }}' . | xargs -0 sd -f c -s "{{ snake_case }}_service" "{{{{ artifact_id }}"
  fd -0 '{{ pattern }}' . | xargs -0 sd -f c -s "{{ PascalCase }}" "{{{{ PrefixName }}"
  fd -0 '{{ pattern }}' . | xargs -0 sd -f c -s "{{ snake_case }}" "{{{{ prefix_name }}"
  fd -0 '{{ pattern }}' . | xargs -0 sd -f c -s {{"{{"}}"{" {{"{{"}}"'{'}}"{{"{{"}}
