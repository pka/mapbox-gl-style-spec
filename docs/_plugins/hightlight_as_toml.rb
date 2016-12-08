require "jekyll/tags/highlight"
require "open3"

module Jekyll
  class HighlightAsTomlBlock < Jekyll::Tags::HighlightBlock

    def initialize(tag_name, markup, tokens)
      @highlighttoml = Liquid::Template.parse("{% highlight0 toml %}{{toml}}{% endhighlight0 %}")
      @highlightjson = Liquid::Template.parse("{% highlight0 json %}{{json}}{% endhighlight0 %}")
      super(tag_name, markup, tokens)
    end

    def render(context)
      if @lang == 'json'
        json = render_all(@nodelist, context)

        jsonpatched = if json =~ /^\b*{/
            json.clone
        elsif json =~ /^\b*\[/
            "{\"filter\": #{json}}"
        else
            "{#{json}}"
        end
        jsonpatched.gsub!('...', '')

        stdin, stdout, stderr = Open3.popen3('cd glstyleconv && cargo run -q')
        stdin.puts(jsonpatched)
        stdin.close
        toml = stdout.read
        toml << stderr.read

        @highlighttoml.registers.merge!(context.registers)
        out = @highlighttoml.render('toml' => toml)

        # Debugging output
        @highlightjson.registers.merge!(context.registers)
        out += @highlightjson.render('json' => json)

        out
      else
        super
      end
    end
  end
end

# Overwrite highlight tag
Liquid::Template.register_tag('highlight', Jekyll::HighlightAsTomlBlock)
Liquid::Template.register_tag('highlight0', Jekyll::Tags::HighlightBlock)
