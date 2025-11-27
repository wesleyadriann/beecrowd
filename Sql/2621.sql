SELECT products.name
FROM products
JOIN providers ON providers.id = products.id_providers
WHERE 
  products.amount >= 10 AND
  products.amount <= 20 AND
  providers.name LIKE 'P%'
