#!/usr/bin/ruby

class MegaGreeter
  attr_accessor :names

  # Create obj
  def initialize(names = "Inno")
    @names = names
  end

  def say_hi
    if @names.nil?
      puts "..."
    elsif @names.respond_to?("each")
      # names is literable
      @names.each do |name|
        puts "Hello #{name}"
      end
    else
      puts "Hello #{names}"
    end
  end

  def say_bye
    if @names.nil?
      puts "..."
    elsif @names.respond_to?("join")
      puts "Goodbye #{@names.join(", ")}. Come back soon!"
    else
      puts "Goodbye #{@names}. Come back soon!"
    end
  end

end

if __FILE__ == $0
  mg = MegaGreeter.new
  mg.say_hi
  mg.say_bye

  mg.names = "Jack Ma"
  mg.say_hi
  mg.say_bye

  mg.names = ["Inno", "Brenda", "Chalers", "Dave"]
  mg.say_hi
  mg.say_bye
end
