# FastWoothee

[![Gem Version](https://badge.fury.io/rb/fast_woothee.svg)](https://badge.fury.io/rb/fast_woothee)
[![Build Status](https://travis-ci.org/ianks/fast_woothee.svg?branch=master)](https://travis-ci.org/ianks/fast_woothee)

Ruby Bindings to [woothee-rust](https://github.com/hhatto/woothee-rust) for
performant, and safe user-agent parsing.

## Installation

Having Rust/Cargo installed is _optional_. If it is not installed, the gem will
download the libs binary automatically. Simply add this line to your
application's Gemfile:

```ruby
gem 'fast_woothee'
```

And then execute:

    $ bundle

Or install it yourself as:

    $ gem install fast_woothee

## Usage

```ruby
FastWoothee.parse 'Mozilla/5.0 (iPad; U; CPU OS 3_2_1 like Mac OS X; en-us) AppleWebKit/531.21.10 (KHTML, like Gecko) Mobile/7B405'
# {:name=>"Webview", :category=>"smartphone", :os=>"iPad", :os_version=>"3.2.1", :browser_type=>"browser", :version=>"UNKNOWN", :vendor=>"OS vendor"}
```

## Development

After checking out the repo, run `bin/setup` to install dependencies. Then, run
`rake spec` to run the tests. You can also run `bin/console` for an interactive
prompt that will allow you to experiment.

To install this gem onto your local machine, run `bundle exec rake install`. To
release a new version, update the version number in `version.rb`, and then run
`bundle exec rake release`, which will create a git tag for the version, push
git commits and tags, and push the `.gem` file to
[rubygems.org](https://rubygems.org).

## Contributing

Bug reports and pull requests are welcome on GitHub at https://github.com/ianks/fast_woothee.
