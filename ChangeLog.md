# Changelog for enum-tools

## 0.5.5 -- 2025-01-28

* Remove const from fn unless it's always there
  Prior some fn's are only const in certain configurations
* Improve documentation

## 0.5.4 -- 2025-01-27

* Use const fn when possible

## 0.5.3 -- 2023-11-04

* Add support to rename fields
* Improve error handling
* Several small optimisations

## 0.5.2 -- 2023-05-17

* Add feature range_fn
* Use syn-2.0

## 0.5.1 -- 2022-10-30

* Fix clippy warnings
* Remove dependency remain
* Remove patch level precision of the dependencies
* Forward ExactSizeIterator::len instead of using the default

## 0.5.0 -- 2022-10-16

* Initial release
