# frozen_string_literal: true
require 'thermite/fiddle'

Thermite::Fiddle.load_module 'initialize_fast_woothee',
                             cargo_project_path: 'ext',
                             ruby_project_path: '.'