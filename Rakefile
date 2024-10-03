# frozen_string_literal: true

require "bundler/gem_tasks"
require "standard/rake"

require "rb_sys/extensiontask"

task build: :compile

GEMSPEC = Gem::Specification.load("actionpack_rust.gemspec")

RbSys::ExtensionTask.new("actionpack_rust", GEMSPEC) do |ext|
  ext.lib_dir = "lib/actionpack_rust"
end

task default: %i[compile standard]
