SELECT categories.name, SUM(products.amount)
FROM categories
JOIN products ON categories.id = products.id_categories
GROUP BY categories.name
