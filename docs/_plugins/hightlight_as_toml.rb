require "jekyll/tags/highlight"
require "open3"

module Jekyll
  class HighlightAsTomlBlock < Jekyll::Tags::HighlightBlock

    def initialize(tag_name, markup, tokens)
      @highlighttoml = Liquid::Template.parse("{% highlight0 toml %}{{toml}}{% endhighlight0 %}")
      @highlightjson = Liquid::Template.parse("{% highlight0 json %}{{json}}{% endhighlight0 %}")
      super(tag_name, markup, tokens)
      @lang = 'toml' if @lang == 'json'
    end

    def render(context)
      if @lang == 'toml'
        json = render_all(@nodelist, context)
        jsonpatched = if json =~ /^\b*{/ then json else "{#{json}}" end
        stdin, stdout, stderr = Open3.popen3('cd glstyleconv && cargo run -q')
        stdin.puts(jsonpatched)
        stdin.close
        toml = (stdout.read || "")

        @highlighttoml.registers.merge!(context.registers)
        out = @highlighttoml.render('toml' => toml)

        # Debugging output
        @highlightjson.registers.merge!(context.registers)
        json << (stderr.read || "")
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
