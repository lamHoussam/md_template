
{{ 
    print '\# ' . $(class) ;;
}}

## Namespace : {{ print $(namespace) ;; }}

## Inherits

{{
    print $(parents) ;;
}}

## Descrition

{{
    print '$(description)' ;;
}}

## Definition

```csharp
{{
    print $(declaration) ;;
}}

```

## Members

{{
    for i in $(variables)
    do
        print '\`' . $(i.name) . '\`' . '(' . $(i.type) . ')' ;;
        print $(i.description) . '\n' ;;
        print '```csharp\n$(i.declaration)\n\n```' ;;
    endfor
}}

## Properties

{{
    for i in $(properties)
    do
        print '\`' . $(i.name) . '\`' . $(i.type) ;;
        print '```csharp\n$(i.declaration)\n\n```' ;;
        print $(i.description) ;;
    endfor
}}

## Methods

{{
    for i in $(methods)
    do
        print '\`' . $(i.name) . '\`' . '(' . $(i.type) . ')' ;;
        print '\* ' . $(i.description) ;;
        print '### Arguments' ;;
        for arg in $(i.arguments)
        do
            print '\`' . $(arg.name) . '\`' . $(arg.type) ;;
            print '  * ' . $(arg.description) ;;
            print '```csharp\n$(arg.declaration)\n\n```' ;;
        endfor
    endfor
}}
